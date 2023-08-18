---
title: "blender basic"
categories: [blender 연구소]
tags: [Blender]
date: 2023-08-18 00:10
img_path: /assets/img/blender/
---

---

## **인터페이스**

![Blender Interface](blender_init.png)

메뉴 라인을 살펴보면 좌측은 기본 메뉴, 중앙에는 워크 스페이스 목록, 그리고 우측에는 Scene 선택창과 View Layer 선택창(주로 렌더링 관련된 설정을 하는 곳)이 있다.

왼쪽 기본 메뉴 관련된 내용은 키보드 숏컷을 사용하는 편이 낫다.

중앙 메뉴에서는 해당 작업 성질에 따라 고를 수 있다. (Layout, Modeling, Sculping, ...)

우측 위는 파일을 관리할 수 있는곳, 우측 아래는 해당 오브젝트에 대한 설정을 할 수 있는 곳이다.

### **키보드 단축키**

[File] 메뉴

- 파일 열기: `Ctrl + O`
- 파일 저장: `Ctrl + S`
- 새 이름으로 저장: `Shift + Ctrl + S`

[Edit] 메뉴

- Undo: `Ctrl + Z`
- Redo: `Shift + Ctrl + Z`
- Rename: `F2`

[Render] 메뉴

- 이미지 렌더링 (현재 프레임만 렌더링): `F12`
- 애니메이션 렌더링 (설정된 구간 전부 렌더링): `Ctrl + F12`

[Window] 메뉴

화면 구성과 관련된 기능들이 있다. (사용할 일이 드물다.)

[Help] 메뉴

Blender와 관련된 문서와 커뮤니티 등이 있다.

## **Workspace에 대해**

> 뭐든지 해당 워크스페이스에서만 "해당" 작업이 가능한 것이 아니다.
{: .prompt-tip }

[Layout] Workspace

처음엔 "Object" 모드로 설정되어 있고, 오브젝트의 이동/회전/크기 조절을 할 수 있다.

[Modeling] Workspace

Viewport가 "Edit" 모드로 설정되어 있어서, 선택한 오브젝트의 메시를 편집할 수 있다.

[Sculpting] Workspace

Viewport가 "Sculpt" 모드로 설정되어 있다. 여기서는 다양한 Sculpting 툴을 사용해서, 메시를 변형시킬 수 있다.

[UV Editing] Workspace

우측의 Edit 모드와 좌측의 UV 에디터로 구성되어 있다.

- 우측의 Edit 모드: 3차원의 메시에 심 라인을 설정할 수 있다.
- 좌측의 UV 에디터: 평면에 펼쳐진 메시의 조각을 배치할 수 있다.

설명하자면, 심 라인이라는 이름의 "칼집"을 내고, Unwrap이라는 펼치는 과정을 거쳐 좌측 UV 에디터에 펼쳐진 조각을 배치하면, 2차원의 이미지를 3차원의 메시에 적용할 수 있다.

[Texture Paint] Workspace

우측의 Viewport Texture Paint Mode에서 3차원 메시에 3D 페인팅을 할 수 있다.

좌측에서는 평면에 펼쳐진 공간에 2D 페인팅을 할 수 있다.

[Shading] Workspace

상단의 Viewport와 하단의 Shader Editor로 구성되어 있다.

하단의 Shader Editor에서 Node를 통해 Material을 꾸밀 수 있고, 상단의 Viewport에서 결과를 확인할 수 있다.

[Animation] Workspace

우측의 Viewport 오브젝트 모드, 좌측의 카메라 뷰, 하단의 Dope Sheet로 구성되어 있다.

- Viewport, Dope Sheet: 오브젝트나 본을 포징하고, 해당 상태를 기록할 수 있다. (키 애니메이션)
- 카메라 뷰: 키 애니메이션의 결과를 확인할 수 있다.

[Rendering] Workspace

렌더링 관련 설정 및 렌더링 결과를 확인할 수 있다.

우측 Propertice Editor의 렌더링 세팅 탭에서, 렌더링과 관련된 설정을 할 수 있고,

좌측 이미지 에디터에서 렌더링 결과를 미리 확인할 수 있다.

이미지 에디터 상단의 "슬롯"을 선택하고 렌더링하면 여러 렌더링의 비교도 가능해진다.

[Compositing] Workspace

렌더링 결과에 포스트 프로세싱, 이펙트를 추가할 수 있다.

또한 Dope Sheet에서 이펙트에 애니메이션을 적용할 수도 있다.

[Geometry Node], [Scripting] Workspace

- 생략 -

---

## **화면 구성 변경**

최상단 및 최하단의 스테이터스바를 제외한 모든 공간은 "에디터"로 구성되어 있다. 또한 각 에디터의 크기와 종류는 마음대로 설정할 수 있다.

앞서 설명했던 워크스페이스는 굳이 거기로 들어가지 않더라도, 똑같이 설정할 수 있다.

[에디터의 종류] 변경하는 방법

좌측 상단 "코너"를 열면 여러 에디터가 나오는데, 하나를 클릭하여 들어가면 된다.

[Area] 분할하는 방법

Area의 코너를 좌클릭한 후, 안쪽으로 드래그하면 된다.

반대로 "병합"하려면, Area의 코너를 좌클릭한 후, 다른 Area로 드롭하면 된다.

참고로 너비가 다른 Area는 병합할 수 없다. (너비를 맞춰줘야 한다. (Width))

[두 Area 교환] 하는 방법

Ctrl을 누른 상태에서 코너를 좌클릭하고, 대상 Area에 드롭하면 된다.

[Area] 독립시키는 방법

Shift를 누른 상태에서, 코너를 좌클릭 -> 드래그 하면 된다.

[Area] 최대화 하는 방법

해당 Area에 마우스 커서를 둔 후, `Ctrl + Space`를 입력하면 된다.

반대의 경우에도 `Ctrl + Space`를 누르면 된다.

---

출처: 블렌더로 만드는 동화같은 3D 캐릭터 - 박영웅
