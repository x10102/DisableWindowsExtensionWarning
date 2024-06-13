from argparse import ArgumentParser
import hashlib

EXPECTED_ORIGINAL_SHA1 = 'd5685def0a05b5c96417d0b39bbf9eadfea7ded5'
EXPECTED_OUTPUT_SHA1 = 'b0197090a6012b2865d7095c2cfc7a2ef8f08f96'
ORIGINAL_BYTES = b'\x85\xc0\x78\x34' # TEST eax,eax; JS 0x34
NEW_BYTES = b'\x48\x90\xeb\x34' # NOP; JMP 0x34
OFFSET = 0x004D0A4C
PATCH_LENGTH = 4

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
    if(hashlib.sha1(data).hexdigest() != EXPECTED_ORIGINAL_SHA1):
        print('ERROR: Hash mismatch. File was probably updated, submit an issue on github.')
        exit(0)
    else:
        print('Hash OK')

    print('Patching...')
    for i in range(PATCH_LENGTH):
        data[OFFSET+i] = NEW_BYTES[i]

    print('Writing output...')
    with open(args.patched, 'wb') as newfile:
        newfile.write(data)

    with open(args.patched, 'rb') as newfile:
        print('Checking hashes...')
        if(hashlib.sha1(newfile.read()).hexdigest() != EXPECTED_OUTPUT_SHA1):
            print('ERROR: Hash mismatch. Unknown error. DO NOT USE THE GENERATED FILE.')
            exit(0)
        else:
            print('Patch done.')