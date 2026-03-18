# 安全改进实施计划

## 概述

本计划旨在实现 `connection.rs` 文件中的三个 TODO 项（去除配置文件权限限制），增强密码等敏感数据的安全性。

***

## \[ ] 任务 1: 添加加密相关依赖和创建安全模块

* **Priority**: P0

* **Depends On**: None

* **Description**:

  * 更新 `Cargo.toml` 添加必要的加密依赖（aes-gcm, argon2, zeroize, rand, base64, chacha20poly1305）

  * 创建 `security.rs` 模块，提供加密/解密功能

* **Success Criteria**:

  * Cargo.toml 更新成功

  * 安全模块文件创建完成

* **Test Requirements**:

  * `programmatic` TR-1.1: Cargo build 无错误

* **Notes**: 使用 Argon2 进行密钥派生，AES-GCM 进行加密

## \[ ] 任务 2: 重构 ConnectionConfig 结构体支持加密密码

* **Priority**: P0

* **Depends On**: 任务 1

* **Description**:

  * 修改 `ConnectionConfig` 结构体，添加加密盐和 nonce 字段

  * 实现密码加密和解密方法

  * 更新 serde 序列化逻辑，确保密码安全存储

* **Success Criteria**:

  * 密码在序列化时自动加密

  * 反序列化时自动解密

* **Test Requirements**:

  * `programmatic` TR-2.1: 加密解密测试通过

  * `programmatic` TR-2.2: 序列化/反序列化测试通过

## \[ ] 任务 3: 实现内存敏感数据清零

* **Priority**: P0

* **Depends On**: 任务 2

* **Description**:

  * 对 `password` 字段实现 `Zeroize` trait

  * 实现 `Drop` trait 在结构体析构时自动清零

* **Success Criteria**:

  * 敏感数据在析构后被清零

* **Test Requirements**:

  * `programmatic` TR-3.1: 内存清零验证测试通过

## \[ ] 任务 4: 更新 ConnectionManager 支持加密存储

* **Priority**: P0

* **Depends On**: 任务 2-3

* **Description**:

  * 更新 `ConnectionManager` 中的保存和加载方法

  * 集成加密功能

* **Success Criteria**:

  * 保存和加载功能正常工作

* **Test Requirements**:

  * `programmatic` TR-4.1: 连接配置保存/加载测试通过

## \[ ] 任务 5: 全面测试与验证

* **Priority**: P0

* **Depends On**: 任务 4

* **Description**:

  * 运行完整测试套件

  * 验证所有安全功能正常工作

* **Success Criteria**:

  * 所有测试通过

  * 安全功能正常

* **Test Requirements**:

  * `programmatic` TR-5.1: `cargo test` 全部通过

  * `programmatic` TR-5.2: `cargo build` 无编译错误

