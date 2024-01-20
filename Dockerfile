FROM --platform=linux/amd64 alpine:latest

WORKDIR /app

COPY ./target/release/dwiz-parachain-node .

# RUN git checkout feature/dwiz-parachain
EXPOSE 30333 9933 9944

ENTRYPOINT ["/bin/sh"]
CMD [ "/app/dwiz-parachain-node --dev" ]