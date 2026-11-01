"""Require representative optimized proof paths to compile to the baseline IR."""
import os
from pathlib import Path
import re
import subprocess
import tempfile
root = Path(__file__).resolve().parent.parent
host = next(line.split(': ', 1)[1] for line in subprocess.check_output(['rustc', '-vV'], text=True).splitlines() if line.startswith('host: '))
subprocess.run(['cargo', 'build', '--release', '--locked', '--offline', '--target', host], cwd=root, check=True)
target = Path(os.environ.get('CARGO_TARGET_DIR', str(root / 'target')))
if not target.is_absolute():
    target = root / target
with tempfile.TemporaryDirectory(prefix='rust-gdp-codegen-') as out:
    ir = Path(out) / 'proofs.ll'
    subprocess.run(['rustc', '--edition=2021', '--crate-type=lib', '-O', '--emit=llvm-ir',
        '--extern', f'rust_gdp={target / host / "release/librust_gdp.rlib"}',
        str(root / 'scripts/codegen.rs'), '-o', str(ir)], check=True)
    text = ir.read_text()
    def body(name, seen=()):
        if name in seen:
            raise AssertionError('alias cycle')
        alias = re.search(r'^@' + name + r' = .*alias[^\n]*@([\w.]+)', text, re.M)
        if alias:
            return body(alias[1], (*seen, name))
        found = re.search(r'^define[^\n]*@' + name + r'\([^\n]*\)[^{]*\{(.*?)^\}', text, re.M | re.S)
        assert found, f'missing function {name}'
        # These fixtures must reduce to returning their sole machine-word argument.
        instructions = [s.strip() for s in found[1].splitlines() if s.strip() and not s.strip().endswith(':') and not s.strip().startswith(';')]
        return instructions
    baseline = body('baseline')
    assert len(baseline) == 1 and baseline[0].startswith('ret i64 '), baseline
    for name in ['scoped', 'refined', 'logical', 'evidence_roundtrip']:
        actual = body(name)
        assert actual == baseline, f'{name}: {actual} != {baseline}'
        print(f'{name}: identical optimized IR to baseline')
