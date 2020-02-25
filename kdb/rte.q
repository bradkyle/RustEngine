interval:0D00:00:05;

upd:insert;
ups:uj

/ get the ticker plant and history ports, defaults are 5010,5012
.u.x:.z.x,(count .z.x)_(":5010";":5012");

/ init schema and sync up from log file;cd to hdb(so client save can run)
.u.rep:{(.[;();:;].)each x;if[null first y;:()];-11!y;system "cd ",1_-10_string first reverse y};
/ HARDCODE \cd if other than logdir/db

/ connect to ticker plant for (schema;(logcount;log))
.u.rep .(hopen `$":",.u.x 0)"(.u.sub[`;`];`.u `i`L)";

piv:{[t;k;p;v]f:{[v;P]`${raze "_" sv x} each string raze P,'/:v};v:(),v; k:(),k; p:(),p;G:group flip k!(t:.Q.v t)k;F:group flip p!t p;key[G]!flip(C:f[v]P:flip value flip key F)!raze{[i;j;k;x;y]a:count[x]#x 0N;a[y]:x y;b:count[x]#0b;b[y]:1b;c:a i;c[k]:first'[a[j]@'where'[b j]];c}[I[;0];I J;J:where 1<>count'[I:value G]]/:\:[t v;value F]};

if[not system"t";system"t 5000";
 .z.ts:{
    trds:0!select first_price:first price, last_price:last price, mean_size:avg size, volume:sum size, min_price:min price, max_price:max price by time:1 xbar interval xbar time, exch, sym from trades;
    bstrds:0!select last_price:last price, min_price:min price, max_price:max price, mean_size:avg size, volume:sum size by time:1 xbar interval xbar time, exch, sym, side from trades;
    dpths:0!select min_size:min size, last_price:last price, last_size: last size by time:1 xbar interval xbar time, exch, sym, side, lvl from depths;
    frts:0!select last funding_rate by time:1 xbar interval xbar time, exch, sym from funding_rates;
    mrkps:0!select first_mark_price:first mark_price, last_mark_price:last mark_price, max_mark_price:max mark_price, min_mark_price:min mark_price by time:1 xbar interval xbar time, exch, sym from mark_prices;

    if[(count trds)>0; (
        ptrds:piv[trds;`time;`exch`sym;`first_price`last_price`mean_size`min_price`max_price`volume];
        show ptrds;
    )];

    if[(count bstrds)>0; (
        bstrds:piv[bstrds;`time;`exch`sym`side;`last_price`mean_size`min_price`max_price`volume];
    )];

    if[(count dpths)>0; (
        dpths:piv[dpths;`time;`exch`sym`side`lvl;`min_size`last_price`last_size];
    )];

    if[(count frts)>0; (
        frts:piv[frts;`time;`exch`sym;`funding_rate];
    )];

    if[(count mrkps)>0; (
        mrkps:piv[mrkps;`time;`exch`sym;`first_mark_price`last_mark_price`max_mark_price`min_mark_price];
    )];

    
    .u.upd[`features; flip features]
 }];