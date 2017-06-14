use std::{mem, ptr};

#[repr(C)]
#[derive(Debug)]
struct BoundaryTag {
    is_alloc: bool,
    is_sentinel: bool,
    free_area_size: usize,
    prev_tag_addr: Option<usize>,
    next_tag_addr: Option<usize>,
}


#[allow(dead_code)]
impl<'a> BoundaryTag {
    fn addr(&self) -> usize
    {
        (self as *const _) as usize
    }

    fn addr_free_area(&self) -> usize
    {
        self.addr() + mem::size_of::<BoundaryTag>()
    }

    fn is_next_of(&self, tag: &'a mut BoundaryTag) -> bool
    {
        match BoundaryTag::next_tag_of(tag) {
            (_, Some(ref v)) if v.addr() == self.addr() => true,
            _  => false,
        }
    }

    fn is_prev_of(&self, tag: &'a mut BoundaryTag) -> bool
    {
        match BoundaryTag::prev_tag_of(tag) {
            (Some(ref v), _) if v.addr() == self.addr() => true,
            _  => false,
        }
    }

    unsafe fn cast_addr_tag_mut(addr: usize) -> &'a mut BoundaryTag
    {
        &mut *(addr as *mut BoundaryTag)
    }

    fn from_memory(addr: usize, size: usize) -> &'a mut BoundaryTag
    {
        let tag = unsafe { BoundaryTag::cast_addr_tag_mut(addr) };

        tag.is_alloc = false;
        tag.is_sentinel = true;
        tag.free_area_size = size - mem::size_of::<BoundaryTag>();
        tag.prev_tag_addr = None;
        tag.next_tag_addr = None;

        tag
    }

    fn divide(tag: &'a mut BoundaryTag, request_size: usize) -> (&'a mut BoundaryTag, Option<&'a mut BoundaryTag>)
    {
        let required_size = request_size + mem::size_of::<BoundaryTag>();
        if tag.free_area_size <= required_size {
            return (tag, None);
        }

        let free_area_size = tag.free_area_size;
        tag.free_area_size = tag.free_area_size - required_size;
        tag.is_sentinel = false;

        // Create new block at the tail of the tag.
        let new_tag_addr = tag.addr_free_area() + free_area_size - required_size;
        tag.next_tag_addr = Some(new_tag_addr);

        let new_tag = BoundaryTag::from_memory(new_tag_addr, required_size);
        new_tag.prev_tag_addr = Some(tag.addr());

        (tag, Some(new_tag))
    }

    // FIXME: This function will cause dangling pointer problems.
    fn merge(tag_x: &'a mut BoundaryTag, tag_y: &'a mut BoundaryTag) -> (&'a mut BoundaryTag)
    {
        // TODO: use Result type.
        let (tag_prev, tag_next) =
            match (tag_x.is_prev_of(tag_y), tag_x.is_next_of(tag_y)) {
                (true, false) => (tag_x, tag_y),
                (false, true) => (tag_y, tag_x),
                _ => panic!("FIXME: to handle the invalid cases"),
            };

        tag_prev.free_area_size += mem::size_of::<BoundaryTag>() + tag_next.free_area_size;
        tag_prev.is_sentinel = tag_next.is_sentinel;
        tag_prev.next_tag_addr = tag_next.next_tag_addr;

        tag_prev
    }

    fn next_tag_of(tag: &'a mut BoundaryTag) -> (&'a mut BoundaryTag, Option<&'a mut BoundaryTag>)
    {
        match tag.next_tag_addr {
            Some(addr) => (tag, Some(unsafe { BoundaryTag::cast_addr_tag_mut(addr) })),
            None       => (tag, None),
        }
    }

    fn prev_tag_of(tag: &'a mut BoundaryTag) -> (Option<&'a mut BoundaryTag>, &'a mut BoundaryTag)
    {
        match tag.prev_tag_addr {
            Some(addr) => (Some(unsafe { BoundaryTag::cast_addr_tag_mut(addr) }), tag),
            None       => (None, tag),
        }
    }
}


fn from_memory<'a>(addr: usize, size: usize) -> &'a mut BoundaryTag
{
    let tag = unsafe { BoundaryTag::cast_addr_tag_mut(addr) };

    tag.is_alloc = false;
    tag.is_sentinel = true;
    tag.free_area_size = size - mem::size_of::<BoundaryTag>();
    tag.prev_tag_addr = None;
    tag.next_tag_addr = None;

    tag
}


fn allocate_memory() -> (usize, usize)
{
    const SIZE: usize = 4096;
    let ref mut x: [u8; SIZE] = unsafe { mem::zeroed() };

    assert_eq!(SIZE, mem::size_of_val(x));

    let addr = (x as *const _) as usize;

    (addr, SIZE)
}


fn main()
{
    let (addr, size) = allocate_memory();
    let tag = from_memory(addr, size);

    let addr = tag.addr_free_area();
    let size = tag.free_area_size;
    println!("free_area_size = {}", size);
    let x = unsafe { &mut *(addr as *mut [u8; 2320 + 1]) };
    let tail_addr = addr + tag.free_area_size;

    let mut last = 0;
    for i in x.iter_mut() {
        *i = 0xCAu8;
        let addr = (i as *const _) as usize;
        last = addr;
        // println!("0x{:x} - {:p} = {:?}: 0x{:x}", tail_addr, i, tail_addr - addr, i);
    }

    let mut cnt = 0;
    for i in x.iter() {
        let addr = (i as *const _) as usize;
        // println!("0x{:x} - {:p} = {:?}, 0x{:x}: 0x{:x}", tail_addr, i, tail_addr - addr, last, i);
        println!("i           = 0x{:x}", i);
        println!("addr_of(i)  = {:p}", i);
        println!("diff        = 0x{:x}", tail_addr - addr);
        println!("last addr   = 0x{:x}", last);
        println!("index       = {:?}", cnt);
        println!("");
        println!("tag addr  = 0x{:x}", tag.addr());
        println!("tail_addr = 0x{:x}", tail_addr);
        println!("{:?}", tag.free_area_size);
        println!("{:?}", tail_addr - tag.addr());
        println!("");
        assert_eq!(*i, 0xCA);
        cnt += 1;
    }

}
