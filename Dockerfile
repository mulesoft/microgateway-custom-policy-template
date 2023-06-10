FROM artifacts.msap.io/mulesoft/api-gateway-base-image-patcher:v1.0.39 AS base

RUN echo "pending"

FROM base as test

RUN echo "pending"

FROM base AS production

RUN echo "pending"
