class Solution
{
    public static void Main(string[] args)
    {
        int n = Convert.ToInt32(Console.ReadLine().Trim());

        List<int> arr = Console.ReadLine().TrimEnd().Split(' ').ToList().Select(arrTemp => Convert.ToInt32(arrTemp)).ToList();
	// Reverse the elements of the array
	arr.Reverse();
	
	// Print out the elements of the array
	Console.Out.NewLine = " ";
	foreach (int i in arr)
	{
	    Console.WriteLine(i);
	}
    }
}
