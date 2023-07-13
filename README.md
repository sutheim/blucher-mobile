This repo is split from blucher main since it differs a lot in target and rust version

This depends on blucher, specifically blucher-data, and expects the following layout:
- parent
    - blucher-mobile
        - src-tauri
    - blucher
        - blucher-data

For mobile development
```bash
npm run tauri android dev
```

For desktop development
```bash
npm run tauri dev
```
