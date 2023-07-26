---
title: "SUI 개발 패턴"
categories: [Web3 연구소]
tags: [Sui, Move]
date: 2023-07-26 00:10
img_path: /assets/img/blockchains/
---

---

![Sui Img](sui_back.png)

Move에서 널리 사용되는 프로그래밍 패턴을 다루며, 아래 내용 중 일부는 오직 `Move`에서만 존재할 수도 있다.

---

## **기능(Capability)**

**Capability**은 객체에 작업을 승인할 수 있는 패턴을 말한다. 가장 일반적인 기능 중 하나는 `TreasuryCap` ([sui::coin](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/coin.md)에 정의됨)이다.

```rust
module examples::item {
    use sui::transfer;
    use sui::object::{Self, UID};
    use std::string::{Self, String};
    use sui::tx_context::{Self, TxContext};

    /// 새 `Item`들을 생성할 수 있는 Capability를 표시하는 타입
    struct AdminCap has key { id: UID }

    /// 사용자 지정 NFT느낌의 타입
    struct Item has key, store { id: UID, name: String }

    /// 모듈 이니셜라이저는 모듈이 퍼블리싱될 때 딱 한 번 호출된다.
    /// 여기서 `AdminCap` 인스턴스를 하나만 생성하여 퍼블리셔에게 보낸다.
    fun init(ctx: &mut TxContext) {
        transfer::transfer(AdminCap {
            id: object::new(ctx)
        }, tx_context::sender(ctx))
    }

    /// 첫 번째 인수로 `AdminCap`이 전달되지 않으면 입력 함수를 호출할 수 없다.
    /// 따라서 `AdminCap`의 소유자만이 이 작업을 수행할 수 있다.
    public entry fun create_and_send(
        _: &AdminCap, name: vector<u8>, to: address, ctx: &mut TxContext
    ) {
        transfer::transfer(Item {
            id: object::new(ctx),
            name: string::utf8(name)
        }, to)
    }
}
```

---

## **증인 (Witness)**

**Witness**는 타입의 소유권을 확인하는데 사용되는 패턴이다. 이를 위해 타입의 `drop` 인스턴스를 전달한다. 코인은 이 구현에 의존한다.

```rust
/// `Witness`로만 인스턴스화할 수 있는 제너릭 타입 
/// `Guardian<T>`를 정의하는 모듈이다.
module examples::guardian {
    use sui::object::{Self, UID};
    use sui::tx_context::TxContext;

    /// 팬텀 매개변수 T는 `create_guardian` 함수에서만 초기화할 수 있다.
    /// 하지만 여기에 전달된 타입에는 `drop`이 있어야 한다.
    struct Guardian<phantom T: drop> has key, store {
        id: UID
    }

    /// 이 함수의 첫 번째 인자는 'drop' 기능을 가진 T 타입의 실제 인스턴스이다. 
    /// 수신되는 즉시 삭제된다.
    public fun create_guardian<T: drop>(
        _witness: T, ctx: &mut TxContext
    ): Guardian<T> {
        Guardian { id: object::new(ctx) }
    }
}

/// `Guardian`을 사용하는 사용자 지정 모듈이다.
module examples::peace_guardian {
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    // `guardian`을 종속성으로 사용한다.
    use 0x0::guardian;

    /// 이 타입은 한 번만 사용하도록 되어있다.
    struct PEACE has drop {}

    /// 모듈 이니셜라이저는 코드가 한 번만 호출되도록 하는 가장 좋은 방법이다.
    /// `Witness` 패턴을 사용하는 것이 가장 좋은 방법이다.
    fun init(ctx: &mut TxContext) {
        transfer::public_transfer(
            guardian::create_guardian(PEACE {}, ctx),
            tx_context::sender(ctx)
        )
    }
}
```

이 패턴은 이 예제에서 사용된다:

- [유동성 풀](https://github.com/MystenLabs/sui/blob/main/sui_programmability/examples/defi/sources/pool.move)
- [규제 코인](https://github.com/MystenLabs/sui/blob/main/sui_programmability/examples/fungible_tokens/sources/regulated_coin.move)

---

## **양도 가능한 증인 (Transferable Witness)**

```rust
/// 이 패턴은 다른 두 가지의 조합을 기반으로 한다: Capability과 Witness.

/// Witness는 주의해야 할 부분이므로 권한이 부여된 사용자에게만 스폰을 허용해야 한다.(이상적으로는 한 번만).
/// 그러나 일부 시나리오에서는 다른 모듈 Y에서 사용하기 위해 모듈 X의 유형 승인이 필요하거나
/// 일정 시간이 지난 후에 승인이 수행되어야 하는 경우도 있다.

/// 이러한 드문 시나리오의 경우 저장소 증인은 완벽한 솔루션이다.
module examples::transferable_witness {
    use sui::transfer;
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};

    /// 이제 Witness에 `Store`(래퍼 안에 저장할 수 있는)가 생겼다.
    struct WITNESS has store, drop {}

    /// 증인 타입을 전달한다. 증인을 얻는 데 딱 한 번만 사용할 수 있다.
    struct WitnessCarrier has key { id: UID, witness: WITNESS }

    /// 모듈 퍼블리셔에게 `WitnessCarrier`를 보낸다.
    fun init(ctx: &mut TxContext) {
        transfer::transfer(
            WitnessCarrier { id: object::new(ctx), witness: WITNESS {} },
            tx_context::sender(ctx)
        )
    }

    /// 캐리어 포장을 풀고(unwrap) 내부의 WITNESS 타입을 가져온다.
    public fun get_witness(carrier: WitnessCarrier): WITNESS {
        let WitnessCarrier { id, witness } = carrier;
        object::delete(id);
        witness
    }
}
```

---

## **핫 포테이토**

**핫 포테이토**는 어빌리티가 없는 구조체의 이름이다. 해당 모듈에서만 패킹과 언패킹이 가능하다. 이 구조체에서는 함수 A가 포테이토를 반환하고, 함수 B가 포테이토를 소비하는 경우, 함수 A 뒤에 함수 B를 호출해야 한다.

```rust
module examples::trade_in {
    use sui::transfer;
    use sui::sui::SUI;
    use sui::coin::{Self, Coin};
    use sui::object::{Self, UID};
    use sui::tx_context::{TxContext};

    /// 시리즈 중 첫번째 휴대폰 모델의 가격
    const MODEL_ONE_PRICE: u64 = 10000;

    /// 두 번째 휴대폰 모델의 가격
    const MODEL_TWO_PRICE: u64 = 20000;

    /// 누군가 존재하지 않는 모델을 구매하려고 할 때를 위해
    const EWrongModel: u64 = 1;

    /// 결제 금액과 가격이 일치하지 않는 경우
    const EIncorrectAmount: u64 = 2;

    /// A phone; 최신 모델로 구매하거나 교환할 수 있다.
    struct Phone has key, store { id: UID, model: u8 }

    /// 지불 가능한 Receipt(영수증). 직접 지불하거나 보상 판매 옵션으로 지불해야 한다.
    /// 보관(stored), 소유(owned) 및 반납(dropped)할 수 없다.
    /// `trade_in`이나 `pay_full` 옵션 중 하나를 선택해서 결제해야 한다.
    struct Receipt { price: u64 }

    /// 먼저 휴대폰을 받고, 결제는 나중에 한다.
    /// `Receipt`를 받는 함수 중 하나에, 해당 `Receipt`를 전달해야 한다.
    /// 이 경우 `pay_full` 또는 `trade_in`이다.
    public fun buy_phone(model: u8, ctx: &mut TxContext): (Phone, Receipt) {
        assert!(model == 1 || model == 2, EWrongModel);

        let price = if (model == 1) MODEL_ONE_PRICE else MODEL_TWO_PRICE;

        (
            Phone { id: object::new(ctx), model },
            Receipt { price }
        )
    }

    /// 휴대폰의 전체 가격을 지불하고 `Receipt`를 소비한다.
    public fun pay_full(receipt: Receipt, payment: Coin<SUI>) {
        let Receipt { price } = receipt;
        assert!(coin::value(&payment) == price, EIncorrectAmount);

        // 간단하게 하기 위해 @examples 계정으로 직접 이체합니다.
        transfer::public_transfer(payment, @examples);
    }

    /// 사용하던 휴대폰을 반납하고 새 휴대폰 가격의 50%를 할인받을 수 있다.
    public fun trade_in(receipt: Receipt, old_phone: Phone, payment: Coin<SUI>) {
        let Receipt { price } = receipt;
        let tradein_price = if (old_phone.model == 1) MODEL_ONE_PRICE else MODEL_TWO_PRICE;
        let to_pay = price - (tradein_price / 2);

        assert!(coin::value(&payment) == to_pay, EIncorrectAmount);

        transfer::public_transfer(old_phone, @examples);
        transfer::public_transfer(payment, @examples);
    }
}
```

이 패턴은 이 예제에서 사용된다:

- [플래시 대출](https://github.com/MystenLabs/sui/blob/main/sui_programmability/examples/defi/sources/flash_lender.move)

---

## **ID 포인터**

ID 포인터는 주 데이터(객체)와 그 접근자/기능을 원본에 연결하여 분리하는 기법이다. 이 패턴을 사용할 수 있는 방향은 몇 가지가 있다:

- 공유 객체에 대한 전송 가능한 기능 발행(예: 공유 객체의 'owner' 필드를 변경하는 TransferCap)
- 동적 데이터와 정적 데이터 분리(예: NFT와 해당 컬렉션 정보)
- 일반 애플리케이션(예: 유동성 풀의 LP 토큰)에서 불필요한 유형 연결(및 증인 요구 사항) 피하기

```rust
/// 이 예제는 Sui에서 간단한 `Lock`과 `Key` 메커니즘을 구현한다. 
/// 여기서 `Lock<T>`는 모든 객체를 포함할 수 있는 공유 객체이고 
/// `Key`는 잠금의 내용에 접근하는 데 필요한 소유 객체이다.
///
/// `Key`는 `ID` 필드를 사용하여 해당 `Lock`에 연결한다.
/// 이 검사를 통해 오프체인에서 대상을 검색할 수 있을 뿐만 아니라 
/// 동적 전송 가능 기능과 'static' 콘텐츠를 분리할 수 있다. 
/// 이 접근법의 또 다른 장점은 대상 자산을 항상 검색할 수 있으며, 
/// `Key`는 다른 객체(예: 마켓플레이스 목록)로 래핑될 수 있다는 것이다.
module examples::lock_and_key {
    use sui::object::{Self, ID, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use std::option::{Self, Option};

    /// 자물쇠가 비어있어 가져갈 것이 없다.
    const ELockIsEmpty: u64 = 0;

    /// 키가 자물쇠와 일치하지 않다.
    const EKeyMismatch: u64 = 1;

    /// 자물쇠에 이미 무언가가 포함되어 있다.
    const ELockIsFull: u64 = 2;

    /// 내부에 콘텐츠를 저장하는 자물쇠!
    struct Lock<T: store + key> has key {
        id: UID,
        locked: Option<T>
    }

    /// 자물쇠(Lock)로 생성된 키는 양도할 수 있으며 자물쇠를 여는 데 필요한 모든 정보가 포함되어 있다.
    struct Key<phantom T: store + key> has key, store {
        id: UID,
        for: ID,
    }

    /// 주어진 키에 대한 자물쇠(Lock)의 ID를 반환합니다.
    public fun key_for<T: store + key>(key: &Key<T>): ID {
        key.for
    }

    /// 공유 객체 내부의 일부 콘텐츠를 잠군다. 
    /// 키가 생성되어 트랜잭션 발신자에게 전송된다. 
    /// 예를 들어, `Coin<SUI>`를 내부에 잠가서 보물 상자로 바꿀 수 있다.
    ///
    /// 발신자는 이 `Lock`에 대한 `Key`를 받는다.
    public entry fun create<T: store + key>(obj: T, ctx: &mut TxContext) {
        let id = object::new(ctx);
        let for = object::uid_to_inner(&id);

        transfer::share_object(Lock<T> {
            id,
            locked: option::some(obj),
        });

        transfer::transfer(Key<T> {
            for,
            id: object::new(ctx)
        }, tx_context::sender(ctx));
    }

    /// Key를 사용하여 공유 객체 내부에 무언가를 잠근다.
    /// 자물쇠(Lock)가 비어 있지 않거나 키가 자물쇠와 일치하지 않으면 중단한다.
    public entry fun lock<T: store + key>(
        obj: T,
        lock: &mut Lock<T>,
        key: &Key<T>,
    ) {
        assert!(option::is_none(&lock.locked), ELockIsFull);
        assert!(&key.for == object::borrow_id(lock), EKeyMismatch);

        option::fill(&mut lock.locked, obj);
    }

    /// 키로 잠금을 해제하고 내용물에 접근한다.
    /// 두 조건이 모두 충족될 때만 호출할 수 있다:
    /// - 키가 자물쇠와 일치한다.
    /// - 자물쇠가 비어 있지 않음
    public fun unlock<T: store + key>(
        lock: &mut Lock<T>,
        key: &Key<T>,
    ): T {
        assert!(option::is_some(&lock.locked), ELockIsEmpty);
        assert!(&key.for == object::borrow_id(lock), EKeyMismatch);

        option::extract(&mut lock.locked)
    }

    /// 자물쇠를 잠금 해제하고 내용을 트랜잭션 발신자에게 전송합니다.
    public fun take<T: store + key>(
        lock: &mut Lock<T>,
        key: &Key<T>,
        ctx: &mut TxContext,
    ) {
        transfer::public_transfer(unlock(lock, key), tx_context::sender(ctx))
    }
}
```

이 패턴은 이 예제에서 사용된다:

- [Lock](https://github.com/MystenLabs/sui/blob/main/sui_programmability/examples/basics/sources/lock.move)
- [Escrow](https://github.com/MystenLabs/sui/blob/main/sui_programmability/examples/defi/sources/escrow.move)
- [Hero](https://github.com/MystenLabs/sui/blob/main/sui_programmability/examples/games/sources/hero.move)
- [Tic Tac Toe](https://github.com/MystenLabs/sui/blob/main/sui_programmability/examples/games/sources/tic_tac_toe.move)
- [경매(Auction)](https://github.com/MystenLabs/sui/blob/main/sui_programmability/examples/nfts/sources/auction.move)

---

참고 자료:

- [Sui Examples](https://examples.sui.io/index.html)
