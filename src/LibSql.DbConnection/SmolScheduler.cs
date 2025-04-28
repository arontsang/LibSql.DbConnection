namespace LibSql.DbConnection;

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Bindings;

public static class SmolScheduler
{

    
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)])]
    internal static void ScheduleRunnable(PortableRunnable runnable, uint isWake)
    {
        Console.WriteLine("Scheduled on Thread {0} with Is Wake {1}", Thread.CurrentThread.ManagedThreadId, isWake != 0);
        if (isWake == 0)
        {
            NativeMethods.run_runnable(runnable);
        }
        else
        {
            ThreadPool.QueueUserWorkItem(static runnable => NativeMethods.run_runnable(runnable), runnable, false);
        }
    }
    
    public static async Task Sleep()
    {
        unsafe VoidTask Impl()
        {
            return NativeMethods.dotnet_sleep(&ScheduleRunnable);
        }
        var task = Impl();
        await task;
    }


}