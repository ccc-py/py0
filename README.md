# py0

Python 直譯器與編譯器實驗專案

## 架構

| 元件 | 描述 |
|------|------|
| **py_full** | Python 直譯器 (使用 Python stdlib `ast`，自己實作執行引擎) |
| **py0c** | Python → QD IR 編譯器 + QD 虛擬機 (Python + C 兩種實作) |

## 快速開始

### py_full 直譯器
```bash
python py_full/py0i.py <script.py> [args...]
```

### py0c 編譯器
```bash
# 編譯 Python 到 QD IR
python py0c/py0c.py <script.py> -o output.qd

# 用 Python VM 執行
python py0c/qd0vm.py output.qd

# 用 C VM 執行
gcc py0c/qd0vm.c -o py0c/qd0vm
./py0c/qd0vm output.qd
```

### 測試
```bash
bash py0c/pytest.sh   # Python VM 測試
bash py0c/ctest.sh    # C VM 測試
```

## 文件

- `_doc/qd0spec.md` - QD 四元組 IR 規格
- `_doc/BNF.md` - py_full 支援的 Python 語法
- `py_full/README.md` - py_full 架構說明
- `py0c/README.md` - py0c 架構說明

## 測試檔案

- `tests/hello.py`, `tests/fact.py` - 共享測試
- `py0c/py/hello.py`, `py/fact.py`, `py/json.py` - py0c 編譯測試

## py_full 語言特色

- 類別與繼承
- 閉包與裝飾器
- 例外處理 (try/catch/finally)
- 列表/字典/集合推導式
- f-string 格式化

## py0c 編譯目標

編譯器可成功編譯以下檔案：
- `py/hello.py`
- `py/fact.py`
- `py/json.py`
- `qd0vm.py` (編譯器本體)
