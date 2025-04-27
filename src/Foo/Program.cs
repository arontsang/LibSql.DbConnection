// See https://aka.ms/new-console-template for more information

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using LibSql.DbConnection.Bindings;
using GCHandle = System.Runtime.InteropServices.GCHandle;


var sleeper = new Sleeper();
await sleeper;

Console.WriteLine("Hello World!");

public class Sleeper()
{
    
    public Foo GetAwaiter() => new Foo();
}

public class Foo : ICriticalNotifyCompletion
{
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)])]
    static void Trampoline(nuint callbackPointer)
    {
        var handle = GCHandle.FromIntPtr((IntPtr)callbackPointer);
        
        if (handle.Target is Action callback)
            callback();
        handle.Free();
    }
    
    public unsafe void OnCompleted(Action continuation)
    {
        var handle = GCHandle.Alloc(continuation);
        NativeMethods.sleep(&Trampoline, (nuint)GCHandle.ToIntPtr(handle));
    }

    public unsafe void UnsafeOnCompleted(Action continuation)
    {
        var handle = GCHandle.Alloc(continuation);
        NativeMethods.sleep(&Trampoline, (nuint)GCHandle.ToIntPtr(handle));
    }

    public bool IsCompleted => false;

    public void GetResult()
    {
        
    }
}