FROM artifacts.msap.io/mulesoft/api-gateway-base-image-patcher:1.0.58 AS base

RUN echo "pending"

FROM base as test

RUN echo "pending"

FROM base AS production

RUN echo "pending"
