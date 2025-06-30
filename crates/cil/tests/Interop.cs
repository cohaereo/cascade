using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Test;

public class Interop
{
    [MethodImpl(MethodImplOptions.InternalCall)]
    public static extern void RuntimeInternalCall(int v);

    [DllImport("TestLibrary", CallingConvention = CallingConvention.Cdecl)]
    public static extern void RuntimeDllImportCall(int v);
}
