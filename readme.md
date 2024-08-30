# Precision Mouse Clicker

This program targets for just one thing: click the mouse within designated timeframe.

## English

### How to run the program

1. `config.toml` should be in the same directory with the `.exe` file
2. config file contains two items. `tgt` & `delay`
3. `tgt` needs to be a string(double quoted) in "YYYY-MM-DD hh:mm:ss" format
4. `delay` is a manual delay offset for time in seconds

### How it works

1. On startup, the program calibrates the target time according to sync info from ntp server
2. Sync formula is `user_tgt_datetime - offset - round_trip_delay/2 + user_delay`
3. It clicks where the mouse is located at the designated time
4. The program checks time every 10 ms when the target time is imminent, so it is guaranteed to click within 10ms.

## 한국어

### 유의사항

1. `config.toml`은 exe 파일과 같은 디렉토리 위치(파일 제목 변경 금지)
2. `config.toml`의 `tgt`을 편집하여 목표 시간 설정 . `delay`는 너무 빠를 경우를 대비하여 넣어 놓음(조정가능)
3. 시간 포맷 바뀌면 안됨(24시간 형식) "YYYY-MM-DD hh:mm:ss" 형식을 맞춰줘야 함.

### 동작 원리

1. 목표 시간 경과 시, 현재 마우스 커서가 위치하는 곳을 클릭
2. 완료 후 20초 정도 후에 종료, 그 전에 꺼도 됨.
3. 프로그램 시작 시 컴터 시간을 ntp 시간과 sync하여 시간 오차에 의한 사고 방지
4. 5초 전부터 10ms 단위로 체크하기 시작하므로 10ms 이내에 클릭이 작동함

