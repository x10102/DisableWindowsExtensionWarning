from argparse import ArgumentParser
import hashlib

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
}


parser = ArgumentParser(prog='WindowsExtWarnPatcher',
                     description='Patches windows.storage.dll to disable the extension warning')
parser.add_argument('original', metavar='[ORIGINAL FILE]')
parser.add_argument('patched', metavar='[PATCHED FILE]')

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