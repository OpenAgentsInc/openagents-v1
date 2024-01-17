<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api.proto

namespace Services.Extism.Proto;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * The message returned in response to a `ListModulesRequest`.
 *
 * Generated from protobuf message <code>ListModulesResponse</code>
 */
class ListModulesResponse extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>repeated .Module modules = 1;</code>
     */
    private $modules;
    /**
     * Generated from protobuf field <code>.Pagination pagination = 2;</code>
     */
    protected $pagination = null;
    /**
     * the full count of results in the database (not the count of this message's
     * `modules`).
     *
     * Generated from protobuf field <code>uint64 total = 3;</code>
     */
    protected $total = 0;
    /**
     * Generated from protobuf field <code>.Sort sort = 4;</code>
     */
    protected $sort = null;
    /**
     * Generated from protobuf field <code>optional .Error error = 5;</code>
     */
    protected $error = null;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type array<\Services.Extism.Proto\Module>|\Google\Protobuf\Internal\RepeatedField $modules
     *     @type \Services.Extism.Proto\Pagination $pagination
     *     @type int|string $total
     *           the full count of results in the database (not the count of this message's
     *           `modules`).
     *     @type \Services.Extism.Proto\Sort $sort
     *     @type \Services.Extism.Proto\Error $error
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Api::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>repeated .Module modules = 1;</code>
     * @return \Google\Protobuf\Internal\RepeatedField
     */
    public function getModules()
    {
        return $this->modules;
    }

    /**
     * Generated from protobuf field <code>repeated .Module modules = 1;</code>
     * @param array<\Services.Extism.Proto\Module>|\Google\Protobuf\Internal\RepeatedField $var
     * @return $this
     */
    public function setModules($var)
    {
        $arr = GPBUtil::checkRepeatedField($var, \Google\Protobuf\Internal\GPBType::MESSAGE, \Services.Extism.Proto\Module::class);
        $this->modules = $arr;

        return $this;
    }

    /**
     * Generated from protobuf field <code>.Pagination pagination = 2;</code>
     * @return \Services.Extism.Proto\Pagination|null
     */
    public function getPagination()
    {
        return $this->pagination;
    }

    public function hasPagination()
    {
        return isset($this->pagination);
    }

    public function clearPagination()
    {
        unset($this->pagination);
    }

    /**
     * Generated from protobuf field <code>.Pagination pagination = 2;</code>
     * @param \Services.Extism.Proto\Pagination $var
     * @return $this
     */
    public function setPagination($var)
    {
        GPBUtil::checkMessage($var, \Services.Extism.Proto\Pagination::class);
        $this->pagination = $var;

        return $this;
    }

    /**
     * the full count of results in the database (not the count of this message's
     * `modules`).
     *
     * Generated from protobuf field <code>uint64 total = 3;</code>
     * @return int|string
     */
    public function getTotal()
    {
        return $this->total;
    }

    /**
     * the full count of results in the database (not the count of this message's
     * `modules`).
     *
     * Generated from protobuf field <code>uint64 total = 3;</code>
     * @param int|string $var
     * @return $this
     */
    public function setTotal($var)
    {
        GPBUtil::checkUint64($var);
        $this->total = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>.Sort sort = 4;</code>
     * @return \Services.Extism.Proto\Sort|null
     */
    public function getSort()
    {
        return $this->sort;
    }

    public function hasSort()
    {
        return isset($this->sort);
    }

    public function clearSort()
    {
        unset($this->sort);
    }

    /**
     * Generated from protobuf field <code>.Sort sort = 4;</code>
     * @param \Services.Extism.Proto\Sort $var
     * @return $this
     */
    public function setSort($var)
    {
        GPBUtil::checkMessage($var, \Services.Extism.Proto\Sort::class);
        $this->sort = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>optional .Error error = 5;</code>
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
     * Generated from protobuf field <code>optional .Error error = 5;</code>
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

