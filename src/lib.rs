#![no_std]
pub static DEFAULT_ROOT_PATHS: &[&[u8]] = &[
    b"home/" as &[u8],
    b"media/",
    b"rustc/",
    b"Users/",
    b"Volumes/",
];
pub static DEFAULT_EXTENSIONS: &[&[u8]] = &[b".rs" as &[u8], b".c", b".h", b".zig"];
pub fn ribose(
    a: &mut [u8],
    mut rand: &mut (dyn FnMut(&mut u8) + '_),
    root_paths: Option<&[&[u8]]>,
    extensions: Option<&[&[u8]]>,
) {
    let root_paths = root_paths.unwrap_or(DEFAULT_ROOT_PATHS);
    let extensions = extensions.unwrap_or(DEFAULT_EXTENSIONS);
    for i in 0..(a.len()) {
        let mut censor = false;
        if a[i] == b'/' {
            for s in root_paths.iter().cloned() {
                if a[(i + 1)..][..(s.len())] == *s
                    || a[(i + 1)..][..(s.len() * 2)]
                        .iter()
                        .cloned()
                        .zip(s.iter().flat_map(|a| [*a, *a]))
                        .enumerate()
                        .all(|(a, (b, c))| a % 2 == 1 || b == c)
                {
                    censor = true;
                }
            }
        }
        if a[i] > b'A' && a[i] < b'Z' && a[i + 1] == b':' && a[i + 2] == b'/' {
            censor = true
        }
        if a[i] > b'A' && a[i] < b'Z' && a[i + 2] == b':' && a[i + 4] == b'/' {
            censor = true
        }
        if censor {
            'a: for j in i..(a.len()) {
                rand(&mut a[j]);
                for s in extensions.iter().cloned() {
                    if a[j..][..(s.len())] == *s
                        || a[j..][..(s.len() * 2)]
                            .iter()
                            .cloned()
                            .zip(s.iter().flat_map(|a| [*a, *a]))
                            .enumerate()
                            .all(|(a, (b, c))| a % 2 == 1 || b == c)
                    {
                        break 'a;
                    }
                }
            }
        }
    }
}
