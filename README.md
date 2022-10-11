# gramine-memory-overwrite-failure-demo

## 环境
- gramine@1.2
- sgx@2.17.100.3

## 快速开始


### 1. 构建镜像
```bash
docker build -t memory-overwrite-failure-demo:0.1 .
```

### 2. 验证可读写非 enclave 的内存

> 本节的命令均以 root 权限运行。

#### 2.1. 启动普通应用

```bash
name=xml-gramine-memory-overwrite-failure-demo

repo_tag=memory-overwrite-failure-demo:0.1

docker rm -f $name

# 将入口命令从 `gramine-sgx hello-world` 改写为 `./hello-world`，以非 enclave 模式启动应用
docker run --rm --name $name $repo_tag ./hello-world
```

启动成功的样例日志如下

```bash
this is Alice
originally foo is 'This is some text from Alice'
Now execute
  sudo ./hacker 1 0x55dc2aaf3039 28
Wait for /tmp/ready.txt
```

#### 2.2. 篡改应用的内存

> 此步骤需要另起终端执行。

```bash
name=xml-gramine-memory-overwrite-failure-demo

cmd=$(docker logs $name 2>&1 | grep 'sudo' | awk '{print $2 " " $3 " " $4 " " $5}')

docker exec -it $name $cmd
```

当前终端的样例日志如下
```bash
./hacker 1 0x55dc2aaf3039 28
opening /proc/1/mem, address is 55dc2aaf3039
string at 0x55dc2aaf3039 in process 1 is:
This is some text from Alice
```

普通应用的终端打印新的 `foo` 字符串如下
```bash
now foo is 'This is some text from Bob:)'
```
可见，普通应用进程的 `foo` 变量已被成功篡改。

综上，普通应用进程的内存可被具有**root 权限**的进程读写。

### 3. 验证无法读写 enclave 的内存

> 本节的命令均以 root 权限运行。

#### 3.1. 启动 enclave

```bash
name=xml-gramine-memory-overwrite-failure-demo

repo_tag=memory-overwrite-failure-demo:0.1

docker rm -f $name

# 注意：默认入口命令为 `gramine-sgx hello-world`
docker run --rm                                   \
  --name $name                                    \
  --device /dev/kmsg:/dev/kmsg                    \
  --device /dev/sgx_enclave:/dev/sgx/enclave      \
  --device /dev/sgx_provision:/dev/sgx/provision  \
  $repo_tag
```

启动成功的样例日志如下

```bash
Gramine is starting. Parsing TOML manifest file, this may take some time...
-----------------------------------------------------------------------------------------------------------------------
Gramine detected the following insecure configurations:

  - sgx.allowed_files = [ ... ]                (some files are passed through from untrusted host without verification)

Gramine will continue application execution, but this configuration must not be used in production!
-----------------------------------------------------------------------------------------------------------------------

this is Alice
originally foo is 'This is some text from Alice'
Now execute
  sudo hacker 1 0x3e7f039 28
Wait for /tmp/ready.txt
```

#### 3.2. 试图篡改 enclave 的内存

> 此步骤需要另起终端执行。

```bash
name=xml-gramine-memory-overwrite-failure-demo

cmd=$(docker logs $name 2>&1 | grep 'sudo' | awk '{print "./"$2 " " $3 " " $4 " " $5}')

docker exec -it $name $cmd
```

当前终端的样例日志如下
```bash
./hacker 1 0x3e7f039 28
opening /proc/1/mem, address is 3e7f039
read failed: read buf: Input/output error (os error 5)
```

enclave 的终端打印的 `foo` 字符串如下
```bash
now foo is 'This is some text from Alice'
```

可见，
- 错误码 `os error 5` 表示**拒绝访问（Access is denied）**，即 enclave 进程的 `foo` 变量所在内存无法被读取；
- enclave 进程的 `foo` 变量没有被篡改。

综上，enclave 外的进程（即使具有 root 权限）无法读写 enclave 的内存。

## TODO
- 为什么 hello/src/bin/hacker.rs:L29 行没有报错。
