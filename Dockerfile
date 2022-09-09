FROM rust:1.63.0

# Download and install Panama JDK
RUN cd /tmp && \
    curl -O https://download.java.net/java/early_access/panama/1/openjdk-19-panama+1-13_linux-x64_bin.tar.gz && \
    tar zxvf openjdk-19-panama+1-13_linux-x64_bin.tar.gz && \
    mv jdk-19 /usr/bin/java

# Set Java home
ENV JAVA_HOME=/usr/bin/java
ENV PATH=$JAVA_HOME/bin:$PATH


