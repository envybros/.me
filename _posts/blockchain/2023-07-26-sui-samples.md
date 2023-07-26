---
title: "SUI 개발 - Nft 및 코인(Erc20) 제작"
categories: [Web3 연구소]
tags: [Sui, Move]
date: 2023-07-26 00:20
img_path: /assets/img/blockchains/
---

---

![Sui Img](sui_back.png)

## **NFT**

Sui에서는 모든 것이 NFT이다. 객체는 고유하고 대체 불가능하며 소유권이 있다. 따라서 기술적으로는 간단한 유형 퍼블리싱으로 충분하다.

```rust
module examples::devnet_nft {
    use sui::url::{Self, Url};
    use std::string;
    use sui::object::{Self, ID, UID};
    use sui::event;
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    /// 누구나 발행할 수 있는 NFT 예시
    struct DevNetNFT has key, store {
        id: UID,
        /// 토큰 이름
        name: string::String,
        /// 토큰 설명
        description: string::String,
        /// 토큰 Url
        url: Url,
        // TODO: 사용자 지정 속성 허용하기
    }

    // ===== Events =====

    struct NFTMinted has copy, drop {
        // NFT의 오브젝트 ID
        object_id: ID,
        // NFT의 창시자
        creator: address,
        // NFT의 이름
        name: string::String,
    }

    // ===== Public view functions =====

    /// NFT's `이름` 가져오기
    public fun name(nft: &DevNetNFT): &string::String {
        &nft.name
    }

    /// NFT's `설명` 가져오기
    public fun description(nft: &DevNetNFT): &string::String {
        &nft.description
    }

    /// NFT's `url` 가져오기
    public fun url(nft: &DevNetNFT): &Url {
        &nft.url
    }

    // ===== Entrypoints =====

    /// 새 devnet_nft 생성
    public entry fun mint_to_sender(
        name: vector<u8>,
        description: vector<u8>,
        url: vector<u8>,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        let nft = DevNetNFT {
            id: object::new(ctx),
            name: string::utf8(name),
            description: string::utf8(description),
            url: url::new_unsafe_from_bytes(url)
        };

        event::emit(NFTMinted {
            object_id: object::id(&nft),
            creator: sender,
            name: nft.name,
        });

        transfer::public_transfer(nft, sender);
    }

    /// `nft`를 `recipient`로 전송
    public entry fun transfer(
        nft: DevNetNFT, recipient: address, _: &mut TxContext
    ) {
        transfer::public_transfer(nft, recipient)
    }

    /// `nft`의 `설명`을 `new_description`로 업데이트한다.
    public entry fun update_description(
        nft: &mut DevNetNFT,
        new_description: vector<u8>,
        _: &mut TxContext
    ) {
        nft.description = string::utf8(new_description)
    }

    /// `nft` 영구삭제
    public entry fun burn(nft: DevNetNFT, _: &mut TxContext) {
        let DevNetNFT { id, name: _, description: _, url: _ } = nft;
        object::delete(id)
    }
}
```

---

## **코인 만들기**

Sui 코인을 발행(publishing)하는 것은 새로운 타입을 발행하는 것만큼이나 간단하다. 그러나 일회성 증인을 사용해야 하므로 약간 까다롭다.

```rust
module examples::mycoin {
    use std::option;
    use sui::coin;
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    /// 코인의 타입 식별자다.
    /// 코인의 타입 태그는 `Coin<package_object::mycoin::MYCOIN>`이다.
    /// 타입 이름이 모듈의 이름과 일치하는지 확인해야 한다.
    struct MYCOIN has drop {}

    /// 모듈 이니셜라이저는 모듈 게시 시 한 번 호출된다. 
    /// 발행자에게 treasury cap이 전송되고, 발행자는 발행과 소각을 제어한다.
    fun init(witness: MYCOIN, ctx: &mut TxContext) {
        let (treasury, metadata) = coin::create_currency(witness, 6, b"MYCOIN", b"", b"", option::none(), ctx);
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury, tx_context::sender(ctx))
    }
}
```

`Coin<T>`는 Sui의 일반적인 코인 구현이다. `TreasuryCap`의 소유자는 코인의 발행과 소각을 제어할 수 있다. 추가 트랜잭션은 `TreasuryCap` 객체를 승인으로 사용하여 `sui::coin::Coin`으로 직접 전송할 수 있다.

---

참고 자료:

- [Sui Examples](https://examples.sui.io/index.html)
