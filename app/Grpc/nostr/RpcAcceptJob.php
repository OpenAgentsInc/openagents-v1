<?php

// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: rpc.proto

namespace App\Grpc\nostr;

use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>RpcAcceptJob</code>
 */
class RpcAcceptJob extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>string jobId = 1;</code>
     */
    protected $jobId = '';

    /**
     * Constructor.
     *
     * @param  array  $data  {
     *                       Optional. Data for populating the Message object.
     *
     * @type string $jobId
     *              }
     */
    public function __construct($data = null)
    {
        \App\Grpc\nostr\GPBMetadata\Rpc::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>string jobId = 1;</code>
     *
     * @return string
     */
    public function getJobId()
    {
        return $this->jobId;
    }

    /**
     * Generated from protobuf field <code>string jobId = 1;</code>
     *
     * @param  string  $var
     * @return $this
     */
    public function setJobId($var)
    {
        GPBUtil::checkString($var, true);
        $this->jobId = $var;

        return $this;
    }
}
