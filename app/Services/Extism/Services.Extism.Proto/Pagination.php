<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api.proto

namespace Services.Extism.Proto;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Control/limit the way results are paginated when working with large
 * responses.
 *
 * Generated from protobuf message <code>Pagination</code>
 */
class Pagination extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>uint32 limit = 1;</code>
     */
    protected $limit = 0;
    /**
     * Generated from protobuf field <code>uint32 offset = 2;</code>
     */
    protected $offset = 0;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type int $limit
     *     @type int $offset
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Api::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>uint32 limit = 1;</code>
     * @return int
     */
    public function getLimit()
    {
        return $this->limit;
    }

    /**
     * Generated from protobuf field <code>uint32 limit = 1;</code>
     * @param int $var
     * @return $this
     */
    public function setLimit($var)
    {
        GPBUtil::checkUint32($var);
        $this->limit = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>uint32 offset = 2;</code>
     * @return int
     */
    public function getOffset()
    {
        return $this->offset;
    }

    /**
     * Generated from protobuf field <code>uint32 offset = 2;</code>
     * @param int $var
     * @return $this
     */
    public function setOffset($var)
    {
        GPBUtil::checkUint32($var);
        $this->offset = $var;

        return $this;
    }

}

