# Deterministic on purpose: a digest-pinned base and no network at build time,
# so the built image id is comparable between the two platforms.
FROM alpine@sha256:4bcff63911fcb4448bd4fdacec207030997caf25e9bea4045fa6c8c44de311d1
RUN echo "workload-matrix" > /etc/wlm && chmod 444 /etc/wlm
CMD ["cat", "/etc/wlm"]
