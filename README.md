# Carpy

A package creation tool for Python inspired by Cargo

## Features

New python packages can be created with the `init` command

```bash
$ ./carpy init testProject
Initializing package: testProject
Done
```

If the specified directory doesn't already exist, it will be created. Assuming the directory is empty, the project will be initialized.

```bash
$ tree ./testProject
testProject/
├── pyproject.toml
├── setup.py
├── src
│   └── testProject
│       ├── __init__.py
│       └── main.py
└── tests
    ├── __init__.py
    └── test_sample.py
```

The project is initialized all the files necessary for a project to be a valid python project, and it can be installed directly using:

```bash
$ pip install ./testProject
```

## TODO Features

* Support authors and licenses in `pyproject.toml`. Currently having issues with serializing maps into toml.
* Sub-command `add <library-name>`: Add a dependency to the project's `requirements.txt` file (and `pyproject.toml` or `setup.cfg`), and install it through pip if needed.
