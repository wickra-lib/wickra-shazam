using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Wickra.Shazam;

/// <summary>Raw P/Invoke surface for the wickra-shazam C ABI.</summary>
internal static partial class Native
{
    internal const string Lib = "wickra_shazam";

    /// <summary>Build a shazam from a spec JSON (NUL-terminated UTF-8). Null on error.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_shazam_new(byte[] specUtf8);

    /// <summary>Free a shazam handle.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial void wickra_shazam_free(IntPtr handle);

    /// <summary>
    /// Apply a command JSON (NUL-terminated UTF-8), writing the response into a
    /// caller-owned buffer. Returns the response length, or a negative error code.
    /// </summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial int wickra_shazam_command(IntPtr handle, byte[] cmdUtf8, byte[]? outBuf, nuint cap);

    /// <summary>The library version as a static NUL-terminated string.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_shazam_version();
}
