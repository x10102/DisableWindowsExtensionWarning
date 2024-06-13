from argparse import ArgumentParser
import hashlib

versions = {
    '6301dfa9754e7edc520c5d9eb3448fa38b3a4c05': {
        'output': 'c429380c8578bdc1e99219403020458b9610a214',
        'offset': 0x004DC63C,
        'patch_length': 4,
        'patch': b'\x48\x90\xeb\x34',
        'version': "Windows 10 Home 22H2 build 19045.4529"
    },
    'd5685def0a05b5c96417d0b39bbf9eadfea7ded5': {
        'output': 'b0197090a6012b2865d7095c2cfc7a2ef8f08f96',
        'offset': 0x004D0A4C,
        'patch_length': 4,
        'patch': b'\x48\x90\xeb\x34',
        'version': "Windows 10 Home 21H1 build 19043.1645"
    }
}

parser = ArgumentParser(prog='WindowsExtWarnPatcher',
                     description='Patches windows.storage.dll to disable the extension warning',
                     epilog='USE AT YOUR OWN RISK')
parser.add_argument('original', metavar='[ORIGINAL FILE]')
parser.add_argument('patched', metavar='[PATCHED FILE]')

if __name__ == '__main__':
    args = parser.parse_args()
    data = b''

    print(f'Reading "{args.original}"...')
    with open(args.original, 'r+b') as dllfile:
        dllfile.seek(0)
        data = bytearray(dllfile.read())

    print('Checking hashes...')
    orig_hash = hashlib.sha1(data).hexdigest()
    if orig_hash not in versions:
        print('ERROR: Hash mismatch. File was probably updated, submit an issue on github.')
        exit(0)
    else:
        current_ver = versions[orig_hash]
        print(f'Hash OK, detected version: {current_ver["version"]}')

    print('Patching...')
    for i in range(current_ver['patch_length']):
        data[current_ver['offset']+i] = current_ver['patch'][i]

    print('Writing output...')
    with open(args.patched, 'wb') as newfile:
        newfile.write(data)

    with open(args.patched, 'rb') as newfile:
        print('Checking hashes...')
        if(hashlib.sha1(newfile.read()).hexdigest() != current_ver['output']):
            print('ERROR: Hash mismatch. Unknown error. DO NOT USE THE GENERATED FILE.')
            exit(0)
        else:
            print('Patch done.')