<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api.proto

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Details about a wasm module, either extracted directly from the binary, or
 * inferred somehow.
 *
 * Generated from protobuf message <code>Module</code>
 */
class Module extends \Google\Protobuf\Internal\Message
{
    /**
     * ID for this module, generated by the database.
     *
     * Generated from protobuf field <code>int64 id = 1;</code>
     */
    protected $id = 0;
    /**
     * sha256 hash of the modules raw bytes
     *
     * Generated from protobuf field <code>string hash = 3;</code>
     */
    protected $hash = '';
    /**
     * function imports called by the module (see:
     * <https://github.com/WebAssembly/design/blob/main/Modules.md#imports)>
     *
     * Generated from protobuf field <code>repeated .Import imports = 4;</code>
     */
    private $imports;
    /**
     * function exports provided by the module (see:
     * <https://github.com/WebAssembly/design/blob/main/Modules.md#exports)>
     *
     * Generated from protobuf field <code>repeated .Export exports = 5;</code>
     */
    private $exports;
    /**
     * size in bytes of the module
     *
     * Generated from protobuf field <code>uint64 size = 6;</code>
     */
    protected $size = 0;
    /**
     * path or locator to the module
     *
     * Generated from protobuf field <code>string location = 7;</code>
     */
    protected $location = '';
    /**
     * programming language used to produce this module
     *
     * Generated from protobuf field <code>.SourceLanguage source_language = 8;</code>
     */
    protected $source_language = 0;
    /**
     * arbitrary metadata provided by the operator of this module
     *
     * Generated from protobuf field <code>map<string, string> metadata = 9;</code>
     */
    private $metadata;
    /**
     * timestamp when this module was loaded and stored
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp inserted_at = 10;</code>
     */
    protected $inserted_at = null;
    /**
     * the interned strings stored in the wasm binary (panic/abort messages, etc.)
     *
     * Generated from protobuf field <code>repeated string strings = 11;</code>
     */
    private $strings;
    /**
     * the cyclomatic complexity
     * (<https://en.wikipedia.org/wiki/Cyclomatic_complexity>) of the instructions
     *
     * Generated from protobuf field <code>optional uint32 complexity = 13;</code>
     */
    protected $complexity = null;
    /**
     * the serialized graph in json format
     *
     * Generated from protobuf field <code>optional bytes graph = 14;</code>
     */
    protected $graph = null;
    /**
     * function hashes
     *
     * Generated from protobuf field <code>map<string, string> function_hashes = 15;</code>
     */
    private $function_hashes;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type int|string $id
     *           ID for this module, generated by the database.
     *     @type string $hash
     *           sha256 hash of the modules raw bytes
     *     @type array<\Import>|\Google\Protobuf\Internal\RepeatedField $imports
     *           function imports called by the module (see:
     *           <https://github.com/WebAssembly/design/blob/main/Modules.md#imports)>
     *     @type array<\Export>|\Google\Protobuf\Internal\RepeatedField $exports
     *           function exports provided by the module (see:
     *           <https://github.com/WebAssembly/design/blob/main/Modules.md#exports)>
     *     @type int|string $size
     *           size in bytes of the module
     *     @type string $location
     *           path or locator to the module
     *     @type int $source_language
     *           programming language used to produce this module
     *     @type array|\Google\Protobuf\Internal\MapField $metadata
     *           arbitrary metadata provided by the operator of this module
     *     @type \Google\Protobuf\Timestamp $inserted_at
     *           timestamp when this module was loaded and stored
     *     @type array<string>|\Google\Protobuf\Internal\RepeatedField $strings
     *           the interned strings stored in the wasm binary (panic/abort messages, etc.)
     *     @type int $complexity
     *           the cyclomatic complexity
     *           (<https://en.wikipedia.org/wiki/Cyclomatic_complexity>) of the instructions
     *     @type string $graph
     *           the serialized graph in json format
     *     @type array|\Google\Protobuf\Internal\MapField $function_hashes
     *           function hashes
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Api::initOnce();
        parent::__construct($data);
    }

    /**
     * ID for this module, generated by the database.
     *
     * Generated from protobuf field <code>int64 id = 1;</code>
     * @return int|string
     */
    public function getId()
    {
        return $this->id;
    }

    /**
     * ID for this module, generated by the database.
     *
     * Generated from protobuf field <code>int64 id = 1;</code>
     * @param int|string $var
     * @return $this
     */
    public function setId($var)
    {
        GPBUtil::checkInt64($var);
        $this->id = $var;

        return $this;
    }

    /**
     * sha256 hash of the modules raw bytes
     *
     * Generated from protobuf field <code>string hash = 3;</code>
     * @return string
     */
    public function getHash()
    {
        return $this->hash;
    }

    /**
     * sha256 hash of the modules raw bytes
     *
     * Generated from protobuf field <code>string hash = 3;</code>
     * @param string $var
     * @return $this
     */
    public function setHash($var)
    {
        GPBUtil::checkString($var, True);
        $this->hash = $var;

        return $this;
    }

    /**
     * function imports called by the module (see:
     * <https://github.com/WebAssembly/design/blob/main/Modules.md#imports)>
     *
     * Generated from protobuf field <code>repeated .Import imports = 4;</code>
     * @return \Google\Protobuf\Internal\RepeatedField
     */
    public function getImports()
    {
        return $this->imports;
    }

    /**
     * function imports called by the module (see:
     * <https://github.com/WebAssembly/design/blob/main/Modules.md#imports)>
     *
     * Generated from protobuf field <code>repeated .Import imports = 4;</code>
     * @param array<\Import>|\Google\Protobuf\Internal\RepeatedField $var
     * @return $this
     */
    public function setImports($var)
    {
        $arr = GPBUtil::checkRepeatedField($var, \Google\Protobuf\Internal\GPBType::MESSAGE, \Import::class);
        $this->imports = $arr;

        return $this;
    }

    /**
     * function exports provided by the module (see:
     * <https://github.com/WebAssembly/design/blob/main/Modules.md#exports)>
     *
     * Generated from protobuf field <code>repeated .Export exports = 5;</code>
     * @return \Google\Protobuf\Internal\RepeatedField
     */
    public function getExports()
    {
        return $this->exports;
    }

    /**
     * function exports provided by the module (see:
     * <https://github.com/WebAssembly/design/blob/main/Modules.md#exports)>
     *
     * Generated from protobuf field <code>repeated .Export exports = 5;</code>
     * @param array<\Export>|\Google\Protobuf\Internal\RepeatedField $var
     * @return $this
     */
    public function setExports($var)
    {
        $arr = GPBUtil::checkRepeatedField($var, \Google\Protobuf\Internal\GPBType::MESSAGE, \Export::class);
        $this->exports = $arr;

        return $this;
    }

    /**
     * size in bytes of the module
     *
     * Generated from protobuf field <code>uint64 size = 6;</code>
     * @return int|string
     */
    public function getSize()
    {
        return $this->size;
    }

    /**
     * size in bytes of the module
     *
     * Generated from protobuf field <code>uint64 size = 6;</code>
     * @param int|string $var
     * @return $this
     */
    public function setSize($var)
    {
        GPBUtil::checkUint64($var);
        $this->size = $var;

        return $this;
    }

    /**
     * path or locator to the module
     *
     * Generated from protobuf field <code>string location = 7;</code>
     * @return string
     */
    public function getLocation()
    {
        return $this->location;
    }

    /**
     * path or locator to the module
     *
     * Generated from protobuf field <code>string location = 7;</code>
     * @param string $var
     * @return $this
     */
    public function setLocation($var)
    {
        GPBUtil::checkString($var, True);
        $this->location = $var;

        return $this;
    }

    /**
     * programming language used to produce this module
     *
     * Generated from protobuf field <code>.SourceLanguage source_language = 8;</code>
     * @return int
     */
    public function getSourceLanguage()
    {
        return $this->source_language;
    }

    /**
     * programming language used to produce this module
     *
     * Generated from protobuf field <code>.SourceLanguage source_language = 8;</code>
     * @param int $var
     * @return $this
     */
    public function setSourceLanguage($var)
    {
        GPBUtil::checkEnum($var, \SourceLanguage::class);
        $this->source_language = $var;

        return $this;
    }

    /**
     * arbitrary metadata provided by the operator of this module
     *
     * Generated from protobuf field <code>map<string, string> metadata = 9;</code>
     * @return \Google\Protobuf\Internal\MapField
     */
    public function getMetadata()
    {
        return $this->metadata;
    }

    /**
     * arbitrary metadata provided by the operator of this module
     *
     * Generated from protobuf field <code>map<string, string> metadata = 9;</code>
     * @param array|\Google\Protobuf\Internal\MapField $var
     * @return $this
     */
    public function setMetadata($var)
    {
        $arr = GPBUtil::checkMapField($var, \Google\Protobuf\Internal\GPBType::STRING, \Google\Protobuf\Internal\GPBType::STRING);
        $this->metadata = $arr;

        return $this;
    }

    /**
     * timestamp when this module was loaded and stored
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp inserted_at = 10;</code>
     * @return \Google\Protobuf\Timestamp|null
     */
    public function getInsertedAt()
    {
        return $this->inserted_at;
    }

    public function hasInsertedAt()
    {
        return isset($this->inserted_at);
    }

    public function clearInsertedAt()
    {
        unset($this->inserted_at);
    }

    /**
     * timestamp when this module was loaded and stored
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp inserted_at = 10;</code>
     * @param \Google\Protobuf\Timestamp $var
     * @return $this
     */
    public function setInsertedAt($var)
    {
        GPBUtil::checkMessage($var, \Google\Protobuf\Timestamp::class);
        $this->inserted_at = $var;

        return $this;
    }

    /**
     * the interned strings stored in the wasm binary (panic/abort messages, etc.)
     *
     * Generated from protobuf field <code>repeated string strings = 11;</code>
     * @return \Google\Protobuf\Internal\RepeatedField
     */
    public function getStrings()
    {
        return $this->strings;
    }

    /**
     * the interned strings stored in the wasm binary (panic/abort messages, etc.)
     *
     * Generated from protobuf field <code>repeated string strings = 11;</code>
     * @param array<string>|\Google\Protobuf\Internal\RepeatedField $var
     * @return $this
     */
    public function setStrings($var)
    {
        $arr = GPBUtil::checkRepeatedField($var, \Google\Protobuf\Internal\GPBType::STRING);
        $this->strings = $arr;

        return $this;
    }

    /**
     * the cyclomatic complexity
     * (<https://en.wikipedia.org/wiki/Cyclomatic_complexity>) of the instructions
     *
     * Generated from protobuf field <code>optional uint32 complexity = 13;</code>
     * @return int
     */
    public function getComplexity()
    {
        return isset($this->complexity) ? $this->complexity : 0;
    }

    public function hasComplexity()
    {
        return isset($this->complexity);
    }

    public function clearComplexity()
    {
        unset($this->complexity);
    }

    /**
     * the cyclomatic complexity
     * (<https://en.wikipedia.org/wiki/Cyclomatic_complexity>) of the instructions
     *
     * Generated from protobuf field <code>optional uint32 complexity = 13;</code>
     * @param int $var
     * @return $this
     */
    public function setComplexity($var)
    {
        GPBUtil::checkUint32($var);
        $this->complexity = $var;

        return $this;
    }

    /**
     * the serialized graph in json format
     *
     * Generated from protobuf field <code>optional bytes graph = 14;</code>
     * @return string
     */
    public function getGraph()
    {
        return isset($this->graph) ? $this->graph : '';
    }

    public function hasGraph()
    {
        return isset($this->graph);
    }

    public function clearGraph()
    {
        unset($this->graph);
    }

    /**
     * the serialized graph in json format
     *
     * Generated from protobuf field <code>optional bytes graph = 14;</code>
     * @param string $var
     * @return $this
     */
    public function setGraph($var)
    {
        GPBUtil::checkString($var, False);
        $this->graph = $var;

        return $this;
    }

    /**
     * function hashes
     *
     * Generated from protobuf field <code>map<string, string> function_hashes = 15;</code>
     * @return \Google\Protobuf\Internal\MapField
     */
    public function getFunctionHashes()
    {
        return $this->function_hashes;
    }

    /**
     * function hashes
     *
     * Generated from protobuf field <code>map<string, string> function_hashes = 15;</code>
     * @param array|\Google\Protobuf\Internal\MapField $var
     * @return $this
     */
    public function setFunctionHashes($var)
    {
        $arr = GPBUtil::checkMapField($var, \Google\Protobuf\Internal\GPBType::STRING, \Google\Protobuf\Internal\GPBType::STRING);
        $this->function_hashes = $arr;

        return $this;
    }

}
