# Completely pivots a table
piv:{[t;k;p;v]f:{[v;P]`${raze "_" sv x} each string raze P,'/:v};v:(),v; k:(),k; p:(),p;G:group flip k!(t:.Q.v t)k;F:group flip p!t p;key[G]!flip(C:f[v]P:flip value flip key F)!raze{[i;j;k;x;y]a:count[x]#x 0N;a[y]:x y;b:count[x]#0b;b[y]:1b;c:a i;c[k]:first'[a[j]@'where'[b j]];c}[I[;0];I J;J:where 1<>count'[I:value G]]/:\:[t v;value F]};

# Pivots based on input
piv:{[t;k;p;v]
    / controls new columns names
    f:{[v;P]`${raze " " sv x} each string raze P};
     v:(),v; k:(),k; p:(),p; / make sure args are lists
     G:group flip k!(t:.Q.v t)k;
     F:group flip p!t p;
     key[G]!flip(C:f[v]P:flip value flip key F)!raze
      {[i;j;k;x;y]
       a:count[x]#x 0N;
       a[y]:x y;
       b:count[x]#0b;
       b[y]:1b;
       c:a i;
       c[k]:first'[a[j]@'where'[b j]];
       c}[I[;0];I J;J:where 1<>count'[I:value G]]/:\:[t v;value F]};
