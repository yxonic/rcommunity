# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.0.3 (2022-06-08)

### New Features

 - <csr-id-511a44816b4be5747b05b3b7b9d5e714e78ce591/> basic marker types

### Bug Fixes

 - <csr-id-b56ab8115a2981ad5fcf42ed96f64367fac8292c/> clean up structure and dependencies

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - clean up structure and dependencies ([`b56ab81`](https://github.comgit//yxonic/rcommunity/commit/b56ab8115a2981ad5fcf42ed96f64367fac8292c))
    - basic marker types ([`511a448`](https://github.comgit//yxonic/rcommunity/commit/511a44816b4be5747b05b3b7b9d5e714e78ce591))
</details>

## v0.0.2 (2022-06-07)

<csr-id-98ca8cf4502e42a5e33a039e1e1d14f8026d93f7/>
<csr-id-24e466f6f2e6a601c2107d6a90ecd6cb1fe48585/>
<csr-id-db4114fb6c80f78c7df914c3660c823974f80e07/>
<csr-id-facdeef3db3d518d9599143d36afc7a785c7d9cf/>
<csr-id-e21df3e74470ed7acab5e77b14cfce82951cd7f6/>
<csr-id-4a174c0cf71c9c8f1b339cf6b998462a3fcd3878/>

### Documentation

 - <csr-id-8844a72abecd777a07ecff4178c8506c95bb73ac/> document store module properly

### Refactor

 - <csr-id-98ca8cf4502e42a5e33a039e1e1d14f8026d93f7/> memory store transaction implemented with condvar
 - <csr-id-24e466f6f2e6a601c2107d6a90ecd6cb1fe48585/> lock the whole transaction in memory store
 - <csr-id-db4114fb6c80f78c7df914c3660c823974f80e07/> use specialization to abstract store steps proof-of-concept
 - <csr-id-facdeef3db3d518d9599143d36afc7a785c7d9cf/> split mod into client, markers and store
 - <csr-id-e21df3e74470ed7acab5e77b14cfce82951cd7f6/> simplify get parameter with Into<T> trait

### Test

 - <csr-id-4a174c0cf71c9c8f1b339cf6b998462a3fcd3878/> memory store basic functionality tests

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 2 calendar days.
 - 232 days passed between releases.
 - 7 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rcommunity_core v0.0.2 ([`814cd6d`](https://github.comgit//yxonic/rcommunity/commit/814cd6da18513378b58fa42b292af0056bd06dc3))
    - document store module properly ([`8844a72`](https://github.comgit//yxonic/rcommunity/commit/8844a72abecd777a07ecff4178c8506c95bb73ac))
    - memory store basic functionality tests ([`4a174c0`](https://github.comgit//yxonic/rcommunity/commit/4a174c0cf71c9c8f1b339cf6b998462a3fcd3878))
    - memory store transaction implemented with condvar ([`98ca8cf`](https://github.comgit//yxonic/rcommunity/commit/98ca8cf4502e42a5e33a039e1e1d14f8026d93f7))
    - lock the whole transaction in memory store ([`24e466f`](https://github.comgit//yxonic/rcommunity/commit/24e466f6f2e6a601c2107d6a90ecd6cb1fe48585))
    - use specialization to abstract store steps proof-of-concept ([`db4114f`](https://github.comgit//yxonic/rcommunity/commit/db4114fb6c80f78c7df914c3660c823974f80e07))
    - split mod into client, markers and store ([`facdeef`](https://github.comgit//yxonic/rcommunity/commit/facdeef3db3d518d9599143d36afc7a785c7d9cf))
    - simplify get parameter with Into<T> trait ([`e21df3e`](https://github.comgit//yxonic/rcommunity/commit/e21df3e74470ed7acab5e77b14cfce82951cd7f6))
</details>

## v0.0.1 (2021-10-18)

<csr-id-923b6d184b7c2af9093d7ddc1a7272d5801b1d72/>
<csr-id-a3f80830f10312a56df07390450609733a59f25d/>
<csr-id-f562ae69bcbeda7f7f44d8d009e8f0584454f110/>
<csr-id-40406892577bb76c7254619176bea6688dcb9ff1/>
<csr-id-3061f5c991aac8e0417b82cf8c6d3f1e4f9c3b68/>

### Refactor

 - <csr-id-923b6d184b7c2af9093d7ddc1a7272d5801b1d72/> reaction create api
 - <csr-id-a3f80830f10312a56df07390450609733a59f25d/> rename query types
 - <csr-id-f562ae69bcbeda7f7f44d8d009e8f0584454f110/> add basic blanket impls
 - <csr-id-40406892577bb76c7254619176bea6688dcb9ff1/> split into three repos

### Chore

 - <csr-id-3061f5c991aac8e0417b82cf8c6d3f1e4f9c3b68/> versioning

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 1 calendar day.
 - 5 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release rcommunity_core v0.0.1, rcommunity_macros v0.0.1, rcommunity v0.0.2 ([`e2e6752`](https://github.comgit//yxonic/rcommunity/commit/e2e6752d3ef5f776683cd16de182584f5d12893a))
    - Release rcommunity_core v0.0.1, rcommunity_macros v0.0.1, rcommunity v0.0.2 ([`e97b2f7`](https://github.comgit//yxonic/rcommunity/commit/e97b2f78ee1741f0a3407625534a0632db059217))
    - versioning ([`3061f5c`](https://github.comgit//yxonic/rcommunity/commit/3061f5c991aac8e0417b82cf8c6d3f1e4f9c3b68))
    - reaction create api ([`923b6d1`](https://github.comgit//yxonic/rcommunity/commit/923b6d184b7c2af9093d7ddc1a7272d5801b1d72))
    - refactor split query ([`924e255`](https://github.comgit//yxonic/rcommunity/commit/924e2551163a0c277b96b5d08f515f4d77b4be95))
    - rename query types ([`a3f8083`](https://github.comgit//yxonic/rcommunity/commit/a3f80830f10312a56df07390450609733a59f25d))
    - add basic blanket impls ([`f562ae6`](https://github.comgit//yxonic/rcommunity/commit/f562ae69bcbeda7f7f44d8d009e8f0584454f110))
    - split into three repos ([`4040689`](https://github.comgit//yxonic/rcommunity/commit/40406892577bb76c7254619176bea6688dcb9ff1))
</details>

