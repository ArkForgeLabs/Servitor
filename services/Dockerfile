ARG TYPE
ARG SCRIPT
ARG PORT

FROM denoland/deno:alpine AS javascript
RUN echo "deno go brr"
WORKDIR /app
USER deno
COPY $SCRIPT .
RUN ls
RUN deno cache main.ts
CMD ["run", "--allow-net", "main.ts"]

FROM alpine:latest AS rust
RUN echo "rust go brr"
WORKDIR /app
COPY $SCRIPT .
RUN chmod +x $SCRIPT
CMD ["./${SCRIPT}"]

FROM ${TYPE} AS final
EXPOSE $PORT


