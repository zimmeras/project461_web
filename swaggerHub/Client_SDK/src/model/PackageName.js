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
 * The PackageName model module.
 * @module model/PackageName
 * @version 2.0.0
 */
export class PackageName {
  /**
   * Constructs a new <code>PackageName</code>.
   * Name of a package.  - Names should only use typical \&quot;keyboard\&quot; characters. - The name \&quot;*\&quot; is reserved. See the &#x60;/packages&#x60; API for its meaning.
   * @alias module:model/PackageName
   * @class
   */
  constructor() {
  }

  /**
   * Constructs a <code>PackageName</code> from a plain JavaScript object, optionally creating a new instance.
   * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
   * @param {Object} data The plain JavaScript object bearing properties of interest.
   * @param {module:model/PackageName} obj Optional instance to populate.
   * @return {module:model/PackageName} The populated <code>PackageName</code> instance.
   */
  static constructFromObject(data, obj) {
    if (data) {
      obj = obj || new PackageName();
    }
    return obj;
  }
}
