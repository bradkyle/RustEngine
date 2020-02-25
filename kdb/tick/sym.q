 
/ use timespan with kdb+tick v2.5 or higher. Prior versions use time type
trades:(
    []time:`timespan$();
    sym:`symbol$();
    exch:`symbol$();
    side:`symbol$();
    price:`float$();
    size:`float$());

depths:(
    []time:`timespan$();
    sym:`symbol$();
    exch:`symbol$();
    side:`symbol$();
    lvl:`int$();
    price:`float$();
    size:`float$());

funding_rates:(
    []time:`timespan$();
    sym:`symbol$();
    exch:`symbol$();
    funding_rate:`float$());

mark_prices:(
    []time:`timespan$();
    sym:`symbol$();
    exch:`symbol$();
    mark_price:`float$());

accounts:(
    []time:`timespan$();
    sym:`symbol$();
    exch:`symbol$();
    balance:`float$();
    available_balance:`float$();
    unrealized_pnl:`float$();
    equity:`float$();
    leverage:`float$();
    margin_balance:`float$();
    maint_margin:`float$();
    short_position:`float$();
    short_liquidation_price:`float$();
    short_average_execution_price:`float$();
    short_realized_pnl:`float$();
    long_position:`float$();
    long_liquidation_price:`float$();
    long_average_execution_price:`float$();
    long_realized_pnl:`float$());

orders:(
    []time:`timespan$();
    sym:`symbol$();
    exch:`symbol$();
    oid:`symbol$();
    side:`symbol$();
    order_type:`int$();
    state:`int$();
    price:`float$();
    size:`int$();
    filled:`int$());

features:(
    []time:`timespan$();
    sym:`symbol$();
    interval:`symbol$();
    scalar:`float$());