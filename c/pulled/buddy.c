/*
 * http://www.sourcecodesworld.com/source/show.asp?ScriptID=1077
 *                       BUDDY SYSTEM CODE
 */

#include<stdio.h>
#include<conio.h>
int tree[2050],i,j,k;
void
segmentalloc(int,int),makedivided(int),makefree(int),printing(int,int);
int place(int),power(int,int);
main()
{
	int totsize,cho,req;
	clrscr();
	for(i=0;i<80;i++) printf("%c",5);
	printf("
		    B U D D Y   S Y S T E M  R E Q U I R E M E N T
S

");
	for(i=0;i<80;i++) printf("%c",5);
	printf("
	*  Enter the Size of the memory  :  ");
	scanf("%d",&totsize);
	clrscr();
	while(1)
	{
		for(i=0;i<80;i++) printf("%c",5);
		printf("
			     B U D D Y   S Y S T E M

");
		for(i=0;i<80;i++) printf("%c",5);
		printf("

	*  1)   Locate the process into the Memory
");
		printf("
	*  2)   Remove the process from Memory
");
		printf("
	*  3)   Tree structure for Memory allocation Map
");
		printf("
	*  4)   Exit


");
		for(i=0;i<80;i++) printf("%c",5);
		printf("

	*  Enter your choice  :  ");
		scanf("%d",&cho);
		switch(cho)
		{
			case 1:
				clrscr();
				printf("
");
				for(i=0;i<80;i++) printf("%c",5);
				printf("
");
				printf("			M E M O R Y   A L L O C A T I O N

");
				for(i=0;i<80;i++) printf("%c",5);
				printf("

	*  Enter the Process size  : ");
				scanf("%d",&req);
				segmentalloc(totsize,req);
				break;
			case 2:
				clrscr();
				printf("
");
				for(i=0;i<80;i++) printf("%c",5);
				printf("
");
				printf("			M E M O R Y   D E A L L O C A T I O N

");
				for(i=0;i<80;i++) printf("%c",5);
				printf("

	*  Enter the process size  :  ");
				scanf("%d",&req);
				makefree(req);
				break;
			case 3:
				clrscr();
				printf("
");
				for(i=0;i<80;i++) printf("%c",5);
				printf("
			M E M O R Y   A L L O C A T I O N   M A
    P

");
				for(i=0;i<80;i++) printf("%c",5);
				printf("

");
				printing(totsize,0);
				printf("

");
				for(i=0;i<80;i++) printf("%c",5);
				getch();
				clrscr();
				break;
			default:
				return;
		}
	}
}

void segmentalloc(int totsize,int request)
{
",2);
		printf("
	*  The system don't have enough free memory
");
		printf("
	*  Suggession  :  Go for VIRTUAL MEMORY

");
		getch();
		return;
	}
	while(1)
	{
		if(request<size && request>(size/2))
			break;
		else
		{
			size/=2;
			flevel++;
		}
	}
	for(i=power(2,flevel)-1;i<=(power(2,flevel+1)-2);i++)
		if(tree[i]==0 && place(i))
		{
			tree[i]=request;
			makedivided(i);
			printf("

	 Result    :     Successful Allocation

");
			break;
		}
	if(i==power(2,flevel+1)-1)
	{
		printf("	%c    Result  :  ");
		printf("
	*  The system don't have enough free memory

");
		printf("
	*  Suggession  :  Go for VIRTUAL Memory Mode

");
	}
}

void makedivided(int node)
{
	while(node!=0)
	{
		node=node%2==0?(node-1)/2:node/2;
		tree[node]=1;
	}
}

int place(int node)
{
	while(node!=0)
	{
		node=node%2==0?(node-1)/2:node/2;
		if(tree[node]>1)
			return 0;
	}
	return 1;
}

void makefree(int request)
{
	int node=0;
	while(1)
	{
		if(tree[node]==request)
			break;
		else
			node++;
	}
	tree[node]=0;
	while(node!=0)
	{
		if(tree[node%2==0?node-1:node+1]==0 && tree[node]==0)
		{
			tree[node%2==0?(node-1)/2:node/2]=0;
			node=node%2==0?(node-1)/2:node/2;
		}
		else break;
	}
}

int power(int x,int y)
{
	int z,ans;
	if(y==0) return 1;
	ans=x;
	for(z=1;z<y;z++)
		ans*=x;
	return ans;
}

void printing(int totsize,int node)
{
	int permission=0,llimit,ulimit,tab;
	if(node==0)
		permission=1;
	else if(node%2==0)
		permission=tree[(node-1)/2]==1?1:0;
	else
		permission=tree[node/2]==1?1:0;
	if(permission)
	{
		llimit=ulimit=tab=0;
		while(1)
		{
			if(node>=llimit && node<=ulimit)
				break;
			else
			{
				tab++;
				printf("       ");
				llimit=ulimit+1;
				ulimit=2*ulimit+2;
			}
		}
		printf(" %d ",totsize/power(2,tab));
		if(tree[node]>1)
			printf("---> Allocated %d
",tree[node]);
		else if(tree[node]==1)
			printf("---> Divided
");
		else printf("---> Free
");
		printing(totsize,2*node+1);
		printing(totsize,2*node+2);
	}
}
