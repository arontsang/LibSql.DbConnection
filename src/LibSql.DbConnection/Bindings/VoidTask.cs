

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace LibSql.DbConnection.Bindings;

partial struct VoidTask : ICriticalNotifyCompletion
{
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)])]
    internal static void CallbackTrampoline(nuint callbackPointer)
    {
        var handle = GCHandle.FromIntPtr((IntPtr)callbackPointer);
        
        if (handle.Target is Action callback)
            callback();
        handle.Free();
    }
    
    public VoidTask GetAwaiter() => this;
    
    
    public void OnCompleted(Action continuation)
    {
        UnsafeOnCompleted(continuation);
    }

    public unsafe void UnsafeOnCompleted(Action continuation)
    {
        Console.WriteLine("Did not complete synchronously, awaiting continuation");
        var handle = GCHandle.Alloc(continuation, GCHandleType.Normal);
        NativeMethods.on_completed_void(
            &SmolScheduler.ScheduleRunnable,
            this,
            &CallbackTrampoline, 
            (nuint)GCHandle.ToIntPtr(handle));
    }

     public bool IsCompleted
     {
         get
         {
             unsafe
             {
                 return (IntPtr)task == IntPtr.Zero;
             }
         }
     }

     public void GetResult()
     {
         Console.WriteLine("Getting result!");
     }
}