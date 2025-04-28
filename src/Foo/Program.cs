// See https://aka.ms/new-console-template for more information


using LibSql.DbConnection;


await SmolScheduler.Sleep();
Console.WriteLine("Scheduled on Thread {0}", Thread.CurrentThread.ManagedThreadId);
Console.WriteLine("Hello World!");

