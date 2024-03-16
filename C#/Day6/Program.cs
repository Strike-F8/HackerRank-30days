using System;
using System.Collections.Generic;
using System.IO;

class Solution {
    static void Main(String[] args) {
	// Get the number of strings
	int numStrings = Convert.ToInt32(Console.ReadLine().Trim());
	List<string> strings = new List<string>();

	// Get the strings
	for (int i = 0; i < numStrings; i++)
	    strings.Add(Console.ReadLine().Trim());    

	// Divide the odd and even characters for each string
	foreach (string s in strings)
	{
	    List<char> evens = new List<char>();
	    List<char> odds = new List<char>();

	    foreach (char c in s)
	    {
		if (evens.Count < odds.Count)
		    evens.Add(c);
		else
		    odds.Add(c);
	    }

	    // Print the results
	    Console.Out.NewLine = "";
	    foreach (char c in odds)
		Console.WriteLine(c);
	    Console.WriteLine(" ");

	    foreach (char c in evens)
		Console.WriteLine(c);
	    Console.WriteLine("\n");
	}
    }
}
