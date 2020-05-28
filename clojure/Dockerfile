FROM oracle/graalvm-ce:19.3.2
# FROM oracle/graalvm-ce:20.1.0
# The binary produced by the 19.3.2 image can be compressed
# slightly more via UPX.

RUN gu install native-image

RUN curl -O https://download.clojure.org/install/linux-install-1.10.1.536.sh && \
    chmod +x linux-install-1.10.1.536.sh && \
    ./linux-install-1.10.1.536.sh

WORKDIR /build
COPY deps.edn /build

RUN clojure -e nil

CMD ["clojure", "-A:native-image"]
