# DisableWindowsExtensionWarning
Stop Windows from showing the annoying warning when changing file extensions

This method works by patching a function in `Windows/System32/windows.storage.dll` to skip over a `ShellMessageBoxW` call and the subsequent check.

# Usage
> [!CAUTION]
> Modifying system files always poses a risk of rendering your system unbootable. Make sure you have an up-to-date restore point, as well as a copy of the original DLL and another bootable drive in case things go wrong.

```cmd
python3 patch.py [original DLL] [new DLL]
```

Then take ownership of `windows.storage.dll` and replace it with the patched file.
