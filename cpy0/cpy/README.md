# cpy0/cpy

`cpy0/cpy` 是獨立於 `cpy0/py` 的 `py + C` 版本。

它的執行鏈如下：

1. `host.py` 用 CPython 啟動 `cpy0i.py`
2. `cpy0i.py` 在 Python 階段呼叫 `cpy0/c/py0i`
3. C host 再執行同一份 `cpy0i.py`
4. 第二階段的 `cpy0i.py` 使用 C runtime 提供的 `run_path()`
5. 最終執行目標 script

簡單說，這版是：

- 第一層用 Python 啟動
- 第二層切到 C runtime
- 形成 `py + C` 的 self-host 鏈

## 使用方式

```bash
cd /Users/Shared/ccc/project/py0
python3 cpy0/cpy/host.py cpy0/cpy/cpy0i.py tests/hello.py
python3 cpy0/cpy/host.py cpy0/cpy/cpy0i.py cpy0/cpy/cpy0i.py tests/fact.py
```

## 測試

```bash
cd /Users/Shared/ccc/project/py0/cpy0/cpy
bash test.sh
```
