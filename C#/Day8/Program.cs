using System.Collections.Generic;
class Solution
{
    public static void Main(string[] args)
    {
	// Get the number of entries in the phonebook
	int num_entries = Convert.ToInt32(Console.ReadLine().Trim());
	
	Dictionary<string, int> phone_book = new Dictionary<string, int>();
	string input = "";

	// Get the entries
	for (int i = 0; i < num_entries; i++)
	{
	    input = Console.ReadLine().Trim();
	    // split the key and value
	    phone_book.Add(input.Split()[0], Convert.ToInt32(input.Split()[1]));
	}

	// look up phone numbers until no more input
	while((input = Console.ReadLine()) != null)
	{
	    if (phone_book.TryGetValue(input, out int number))
		Console.WriteLine($"{input}={number}");
	    else
		Console.WriteLine("Not found");
	}
    }
}
