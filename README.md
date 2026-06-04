# gitx

git branch workflow support CLI tool.

## Install

### macOS

#### 1. Download binary from GitHub Releases

#### 2. Create local bin directory

```bash
mkdir -p ~/.local/bin
```

#### 3. Move binary

```bash
mv gitx ~/.local/bin/
```

#### 4. Add PATH

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### 5. Verify

```bash
gitx history
```

---

## Build from source

```bash
cargo build --release
```

Binary:

```text
target/release/gitx
```

---

## Commands

### Create branch

```bash
gitx branch feat 123 add-login
```

### Delete branch

```bash
gitx delete feat/#123-add-login
```

### Show history

```bash
gitx branch feat 10 improve-history-display
gitx delete feat/#10-improve-history-display
gitx history
gitx history -l 5
gitx history -f branch
gitx history -l 5 -f delete
```
