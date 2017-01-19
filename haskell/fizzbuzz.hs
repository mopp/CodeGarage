{- # OPTIONS -Wall -Werror #-}

{- 3 -> fizz -}
{- 5 -> buzz -}
{- 3 && 5 -> fizzbuzz -}

fizzbuzz1 :: Int -> String
fizzbuzz1 x =
    if ((x `mod` 3) == 0) && ((x `mod` 5) == 0) then
        "FizzBuzz"
    else
        if (x `mod` 3) == 0 then
            "Fizz"
        else
            if (x `mod` 5) == 0 then
                "Buzz"
            else
                show x


fizzbuzz2 :: Int -> String
fizzbuzz2 x = fizzbuzz2Sub1 ((x `mod` 3 == 0), (x `mod` 5 == 0), x)
fizzbuzz2Sub1 (True,  True,  _) = "FizzBuzz"
fizzbuzz2Sub1 (True,  False, _) = "Fizz"
fizzbuzz2Sub1 (False, True,  _) = "Buzz"
fizzbuzz2Sub1 (False, False, x) = (show x)


fizzbuzz3 :: Int -> String
fizzbuzz3 x
     | ((x `mod` 3) == 0) && ((x `mod` 5) == 0) = "FizzBuzz"
     | ((x `mod` 3) == 0)                       = "Fizz"
     | ((x `mod` 5) == 0)                       = "Buzz"
     | otherwise                                = show x


fizzbuzz4 :: Int -> String
fizzbuzz4 x
     | (isMod3) && (isMod5) = "FizzBuzz"
     | (isMod3)             = "Fizz"
     | (isMod5)             = "Buzz"
     | otherwise            = show x
     where  isMod3 = (x `mod` 3) == 0
            isMod5 = (x `mod` 5) == 0


doFizzbuzz :: (Int, [Int]) -> [String]
getFunc 1 = fizzbuzz1
getFunc 2 = fizzbuzz2
getFunc 3 = fizzbuzz3
getFunc 4 = fizzbuzz4
getFunc _ = fizzbuzz1
doFizzbuzz (n, nums) = [ (getFunc n) x | x <- nums ]
