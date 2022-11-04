pub const MAIN_CONTENTS: &[u8] =
b"def main():
    print(\"Hello, world!\")

if __name__ == \"__main__\":
    main()
";

pub const TEST_SAMPLE_CONTENTS: &[u8] =
b"import pytest

def inc(x):
    return x + 1

@pytest.mark.xfail(reason = \"Bug with arithmetic\")
def test_answer():
    assert inc(3) == 5
";

pub const SETUP_PY_CONTENTS: &[u8] =
b"from setuptools import setup

if __name__ == \"__main__\":
    setup()
";