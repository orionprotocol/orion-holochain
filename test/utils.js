const createBroker = function(name) {
  return async caller =>
    await caller.callSync("vc", "create_broker", {
      branch_address: branchAddress
    });
};

const createOrder = function(exchangeAddress, brokerAddress, baseAssetCode, quotedAssetCode, direction, quotedPricePerUnit, amount) {
  return async caller =>
    await caller.callSync("vc", "create_order", {
      exchange_addr: exchangeAddress,
      broker_addr: brokerAddress,
      base_asset_code: baseAssetCode,
      quoted_asset_code: quotedAssetCode,
      direction: direction,
      quoted_price_per_unit: quotedPricePerUnit,
      amount: amount
    });
};

const createTrade = function(orderAddress, price, assetCode) {
  return async caller =>
    await caller.callSync("vc", "create_trade", {
      order_addr: orderAddress,
      price: assetAmount,
      assetCode: message
    });
};

module.exports = {
  createBroker,
  createTransaction,
  createOrder
}