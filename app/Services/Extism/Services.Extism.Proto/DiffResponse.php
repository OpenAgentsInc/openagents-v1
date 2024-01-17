<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api.proto

namespace Services.Extism.Proto;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * The message returned in response to `DiffRequest`, contains a text representation of the difference
 * between the two specified modules.
 *
 * Generated from protobuf message <code>DiffResponse</code>
 */
class DiffResponse extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>string diff = 1;</code>
     */
    protected $diff = '';
    /**
     * Generated from protobuf field <code>optional .Error error = 2;</code>
     */
    protected $error = null;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type string $diff
     *     @type \Services.Extism.Proto\Error $error
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Api::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>string diff = 1;</code>
     * @return string
     */
    public function getDiff()
    {
        return $this->diff;
    }

    /**
     * Generated from protobuf field <code>string diff = 1;</code>
     * @param string $var
     * @return $this
     */
    public function setDiff($var)
    {
        GPBUtil::checkString($var, True);
        $this->diff = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>optional .Error error = 2;</code>
     * @return \Services.Extism.Proto\Error|null
     */
    public function getError()
    {
        return $this->error;
    }

    public function hasError()
    {
        return isset($this->error);
    }

    public function clearError()
    {
        unset($this->error);
    }

    /**
     * Generated from protobuf field <code>optional .Error error = 2;</code>
     * @param \Services.Extism.Proto\Error $var
     * @return $this
     */
    public function setError($var)
    {
        GPBUtil::checkMessage($var, \Services.Extism.Proto\Error::class);
        $this->error = $var;

        return $this;
    }

}

