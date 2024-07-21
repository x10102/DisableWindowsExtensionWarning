from argparse import ArgumentParser
import hashlib
import ctypes
import subprocess
import os.path

def green(text):
    return f'\033[0;32m{text}\033[0m'

def red(text):
    return f'\033[0;31m{text}\033[0m'

def yellow(text):
    return f'\033[0;33m{text}\033[0m'

logo = f"""
╔══════════════════════════════════════════════════════════════════╗
║  ██████ ██████   █████   ██████ ██   ██ ████████ ███████ ██   ██ ║
║ ██      ██   ██ ██   ██ ██      ██  ██     ██    ██      ██  ██  ║
║ ██      ██████  ███████ ██      █████      ██    █████   █████   ║
║ ██      ██   ██ ██   ██ ██      ██  ██     ██    ██      ██  ██  ║
║  ██████ ██   ██ ██   ██  ██████ ██   ██    ██    ███████ ██   ██ ║
╠══════════════════════════════════════════════════════════════════╣
║ => https://github.com/x10102/DisableWindowsExtensionWarning      ║
║ => {yellow("USE AT YOUR OWN RISK!")}                                         ║
╚══════════════════════════════════════════════════════════════════╝
"""


versions = {
    '6301dfa9754e7edc520c5d9eb3448fa38b3a4c05': {
        'output': '2c0b7460b796422e612443a080c1c823fb55106f',
        'patches': [
            {
                "offset": 0x004DC663,
                "length": 7,
                "data": b'\x90'*7
            },
            {
                "offset": 0x004DC68D,
                "length": 10,
                "data": b'\x90'*10
            }
        ],
        'version': "Windows 10 Home 22H2 build 19045.4529"
    },
    'bc4f0e77101e62f17a83112378eed57ab32590db': {
        'output': '985aaf2d470339622dfd0c0535b3a6db67223e36',
        'patches': [
            {
                "offset": 0x004DCEB3,
                "length": 7,
                "data": b'\x90'*7
            },
            {
                "offset": 0x004DCEDD,
                "length": 10,
                "data": b'\x90'*10
            }
        ],
        'version': "Windows 10 Home 22H2 build 19045.4651"
    },
}


parser = ArgumentParser(prog='WindowsExtWarnPatcher',
                     description='Patches windows.storage.dll to disable the extension warning')
parser.add_argument('original', metavar='[ORIGINAL FILE]')
parser.add_argument('patched', metavar='[PATCHED FILE]')

def rollback_permissions():
    print("=> Restoring permissions")
    run_command(['icacls', 'C:\\Windows\\System32\\windows.storage.dll', '/setowner', '"NT SERVICE\TrustedInstaller"'])
    run_command(['icacls', 'C:\\Windows\\System32\\windows.storage.dll', '/grant:r', 'Administrators:RX'])

def run_command(command) -> bool:
    try:
        print(f"=> RUN: {' '.join(command)}")
        subprocess.run(command, check=True, shell=True)
        return True
    except subprocess.CalledProcessError as e:
        print(red(f'=> Command failed: {str(e)}'))
        return False

if __name__ == '__main__':
    args = parser.parse_args()
    data = b''
    print(logo)
    print(f'=> Reading "{args.original}"...')
    with open(args.original, 'r+b') as dllfile:
        dllfile.seek(0)
        data = bytearray(dllfile.read())

    print('=> Checking hashes...')
    orig_hash = hashlib.sha1(data).hexdigest()
    if orig_hash not in versions:
        print(red('=> ERROR: Hash mismatch. File was probably updated, submit an issue on github.'))
        exit(0)
    else:
        current_ver = versions[orig_hash]
        print(f'=> Hash OK, detected version: {current_ver["version"]}')

    for patch_idx, patch in enumerate(current_ver['patches']):
        print(yellow(f'=> Patching {patch_idx+1} of {len(current_ver["patches"])}...'))
        for i in range(patch['length']):
            data[patch['offset']+i] = patch['data'][i]

    print('=> Writing output...')
    with open(args.patched, 'wb') as newfile:
        newfile.write(data)

    with open(args.patched, 'rb') as newfile:
        print('=> Checking hashes...')
        if(hashlib.sha1(newfile.read()).hexdigest() != current_ver['output']):
            print(red('=> ERROR: Hash mismatch. Unknown error. DO NOT USE THE GENERATED FILE.'))
            exit(0)
        else:
            print(green('=> Patcher done.'))

    #if ctypes.windll.shell32.IsUserAnAdmin() != 1:
    #    print(yellow("=> Insufficient permissions"))
    #    print(yellow("=> Either replace the file manually or run the script as admin."))
    #else:
    #    print("=> Running as admin")

    #print("=> Taking ownership of windows.storage.dll")
    #run_command(['takeown', '/F', 'C:\\Windows\\System32\\windows.storage.dll', '/A'])
    #run_command(['icacls', 'C:\\Windows\\System32\\windows.storage.dll', '/grant', 'Administrators:F'])

    #print("=> Copying patched DLL")
    #if not run_command(['copy', '/B', '/Y', os.path.abspath(args.patched), 'C:\\Windows\\System32\\windows.storage.dll']):
    #    print(red("=> Copy failed. Reboot and try again or replace manually"))

    #rollback_permissions()

    #print(green("=> All done. Please reboot now."))
    exit(0)