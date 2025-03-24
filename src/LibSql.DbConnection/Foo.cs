using System.Runtime.InteropServices;


namespace LibSql.DbConnection;

using LibSql.DbConnection.Bindings;

public static class Foo
{
    public static int DoFoo(int x, int y)
    {
        Console.WriteLine("Hello World!");
        return NativeMethods.my_add(x, y);
    }
    
    public static unsafe void DoBar()
    {
        var foo = NativeMethods.create_context();
        try
        {
            var bar = new Span<Context>(foo, 1)[0];
        }
        finally
        {
            NativeMethods.delete_context(foo);
        }
        
    }
    
    public static unsafe void DoQux()
    {
        var foo = NativeMethods.create_context();
        try
        {
            var bar = new Span<Context>(foo, 1)[0];
        }
        finally
        {
            NativeMethods.delete_context(foo);
        }
        
    }
}