[//]: # (Incorrect links as they are ascii docs. TODO W-12225942: Add correct links to DW documentation)

# DataWeave Support in Flex Gateway Policies

[DataWeave](https://docs.mulesoft.com/dataweave) is the programming language designed by MuleSoft for data transformation. It enables you to build a simple solution for a common integration developer use case: read and parse data from one format, transform the data, and write it out as a different format.

Flex Gateway supports a subset of DataWeave in policy configuration expressions.

# Available Types

-   [`Null`](https://docs.mulesoft.com/dataweave/latest/dataweave-type-system#null)

-   `Boolean`

-   `String`

-   `Number` (Driven as 64 bits floating points)

-   [`Array`](https://docs.mulesoft.com/dataweave/latest/dataweave-type-system#array-type)

-   [`Object`](https://docs.mulesoft.com/dataweave/latest/dataweave-type-system#object-type) (Repeated keys are not available)

# Available Value Constructors for Types

-   [`Boolean`](https://docs.mulesoft.com/dataweave/latest/dataweave-types#dw_type_boolean)

-   [`String`](https://docs.mulesoft.com/dataweave/latest/dataweave-types#dw_type_string)

-   [`Number`](https://docs.mulesoft.com/dataweave/latest/dataweave-types#dw_type_number)

-   [`Array`](https://docs.mulesoft.com/dataweave/latest/dataweave-types#dw_type_array)

# Unavailable Value Constructors for Types

-   [`Null`](https://docs.mulesoft.com/dataweave/latest/dataweave-types#dw_type_null) (Writing a `null` literal is not supported)

-   [`Object`](https://docs.mulesoft.com/dataweave/latest/dataweave-types#dw_type_object) (Objects can not be defined as literals)

# Available Flow Control Structures

-   [`if else`](https://docs.mulesoft.com/dataweave/latest/dataweave-flow-control#control_flow_if_else)

# Unavailable Flow Control Structures

-   [`do`](https://docs.mulesoft.com/dataweave/latest/dataweave-flow-control#control_flow_do)

# Available Selectors

-   `Array[Number]`

-   `Array[String]`

-   `Object[String]`

-   `String[Number]`

# Unavailable Selectors

-   `Object[Number]`

# Available Equality and Relational Operators

-   [`==`](https://docs.mulesoft.com/dataweave/latest/dw-operators#equality-and-relational-operators)

-   [`!=`](https://docs.mulesoft.com/dataweave/latest/dw-operators#equality-and-relational-operators)

-   [`>=`](https://docs.mulesoft.com/dataweave/latest/dw-operators#equality-and-relational-operators)

-   [`<=`](https://docs.mulesoft.com/dataweave/latest/dw-operators#equality-and-relational-operators)

# Available Logical Operators

-   [`not`](https://docs.mulesoft.com/dataweave/latest/dw-operators#logical_operators)

-   [`and`](https://docs.mulesoft.com/dataweave/latest/dw-operators#logical_operators)

-   [`or`](https://docs.mulesoft.com/dataweave/latest/dw-operators#logical_operators)

# Available Functions

## `++`

-   [`++(Array<S>, Array<T>): Array<S | T>`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-plusplus#plusplus1)

-   [`++(String, String): String`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-plusplus#plusplus2)

## `contains`

-   [`contains(Array<T>, Any): Boolean`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-contains#contains1)

-   [`contains(String, String): Boolean`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-contains#contains2)

## `lower`

-   [`lower(String): String`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-lower#lower1)

-   [`lower(Null): Null`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-lower#lower2)

## `splitBy`

-   [`splitBy(String, String): Array<String>`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-splitby#splitby2)

## `sizeOf`

-   [`sizeOf(Array<Any>): Number`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-sizeof#sizeof1)

-   [`sizeOf(Object): Number`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-sizeof#sizeof2)

-   [`sizeOf(String): Number`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-sizeof#sizeof4)

## `trim`

-   [`trim(String): String`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-trim#trim1)

-   [`trim(Null): Null`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-trim#trim2)

## `upper`

-   [`upper(String): String`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-upper#upper1)

-   [`upper(Null): Null`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-upper#upper2)

## `uuid`

-   [`uuid(): String`](https://docs.mulesoft.com/dataweave/latest/dw-core-functions-uuid#uuid1)

## `dw::core::Strings::substringAfter`

-   [`substringAfter(String, String): String`](https://docs.mulesoft.com/dataweave/latest/dw-strings-functions-substringafter#substringafter1)

-   [`substringAfter(Null, String): Null`](https://docs.mulesoft.com/dataweave/latest/dw-strings-functions-substringafter#substringafter2)

## `dw::core::Strings::substringAfterLast`

-   [`substringAfterLast(String, String): String`](https://docs.mulesoft.com/dataweave/latest/dw-strings-functions-substringafterlast#substringafterlast1)

-   [`substringAfterLast(Null, String): Null`](https://docs.mulesoft.com/dataweave/latest/dw-strings-functions-substringafterlast#substringafterlast2)

## `dw::core::Strings::substringBefore`

-   [`substringBefore(String, String): String`](https://docs.mulesoft.com/dataweave/latest/dw-strings-functions-substringbefore#substringbefore1)

-   [`substringBefore(Null, String): Null`](https://docs.mulesoft.com/dataweave/latest/dw-strings-functions-substringbefore#substringbefore2)

## `dw::core::Strings::substringBeforeLast`

-   [`substringBeforeLast(text: String, separator: String): String`](https://docs.mulesoft.com/dataweave/latest/dw-strings-functions-substringbeforelast#substringbeforelast1)

-   [`substringBeforeLast(text: Null, separator: String): Null`](https://docs.mulesoft.com/dataweave/latest/dw-strings-functions-substringbeforelast#substringbeforelast2)

# Available Predefined Variables

-   [`attributes`](https://docs.mulesoft.com/dataweave/latest/dataweave-variables-context)

    -   `attributes.headers`

    -   `attributes.method` (Only available in request context)

    -   `attributes.queryParams` (Only available in request context)

    -   `attributes.queryString` (Only available in request context)

    -   `attributes.requestPath` (Only available in request context)

    -   `attributes.requestUri` (Only available in request context)

    -   `attributes.localAddress` (Only available in request context)

    -   `attributes.remoteAddress` (Only available in request context)

    -   `attributes.scheme` (Only available in request context)

    -   `attributes.version` (Only available in request context)

    -   `attributes.statusCode` (Only available in response context)

-   [`authentication`](https://docs.mulesoft.com/dataweave/latest/dataweave-variables-context)

    -   `authentication.clientId`

    -   `authentication.clientName`

    -   `authentication.principal`

    -   `authentication.properties`
