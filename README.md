# DisableWindowsExtensionWarning
Stop Windows from showing the annoying warning when changing file extensions

This method works by patching a function in `Windows/System32/windows.storage.dll` to skip over a `ShellMessageBoxW` call and the subsequent check.

Tested on Windows 10 21H1, build 19043.1645

# Usage
> [!CAUTION]
> Modifying system files always poses a risk of rendering your system unbootable. Make sure you have an up-to-date restore point, as well as a copy of the original DLL and another bootable drive in case things go wrong.

```cmd
python3 patch.py [original DLL] [new DLL]
```

Then take ownership of `windows.storage.dll` and replace it with the patched file (You should probably be in safe mode or booted from an installer ISO). Make sure to change the owner back to `TrustedInstaller` and set the permissions exactly as they were originally after you're done.
