/*
 * ECE 461 - Spring 2023 - Project 2
 * API for ECE 461/Spring 2023/Project 2: A Trustworthy Module Registry
 *
 * OpenAPI spec version: 2.0.0
 * Contact: davisjam@purdue.edu
 *
 * NOTE: This class is auto generated by the swagger code generator program.
 * https://github.com/swagger-api/swagger-codegen.git
 *
 * Swagger Codegen version: 3.0.41
 *
 * Do not edit the class manually.
 *
 */
import {ApiClient} from '../ApiClient';

/**
 * The AuthenticationToken model module.
 * @module model/AuthenticationToken
 * @version 2.0.0
 */
export class AuthenticationToken {
  /**
   * Constructs a new <code>AuthenticationToken</code>.
   * The spec permits you to use any token format you like. You could, for example, look into JSON Web Tokens (\&quot;JWT\&quot;, pronounced \&quot;jots\&quot;): https://jwt.io.
   * @alias module:model/AuthenticationToken
   * @class
   */
  constructor() {
  }

  /**
   * Constructs a <code>AuthenticationToken</code> from a plain JavaScript object, optionally creating a new instance.
   * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
   * @param {Object} data The plain JavaScript object bearing properties of interest.
   * @param {module:model/AuthenticationToken} obj Optional instance to populate.
   * @return {module:model/AuthenticationToken} The populated <code>AuthenticationToken</code> instance.
   */
  static constructFromObject(data, obj) {
    if (data) {
      obj = obj || new AuthenticationToken();
    }
    return obj;
  }
}
