# backup-gather

A smarter and faster `find` for listing the files that should be backed up.

This was just written for my own backups, as the `find` command got too complex
with criteria like "if this directory has a file `Cargo.toml`, then don't
back up the `target` subdirectory".

Feel free to use, but it was really just made for me. If you like and and want
features, I'm open to PRs.

## Example use

```
$ backup-gather ~ \
    | tar cf - --no-recursion --null --files-from=- \
    | gpg -e -r foo@example.com \
    > /mnt/backup/$(date +%Y-%m-%d).tar.gpg
```
