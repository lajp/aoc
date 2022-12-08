program part1;

var trees: array[0..100000] of char;
var rowlen, rowc, i, m, k, maxscore, score, sidescore: int64;
var c: char;

begin
    i := 0;
    rowlen := 0;
    maxscore := 0;

    while not eof do
    begin
        read(c);
        if c = #10 then
        begin
            if rowlen = 0 then
                rowlen := i;
            continue
        end;

        trees[i] := c;
        inc(i)
    end;

    rowc := i div rowlen;

    for i := 0 to (rowlen*rowc)-1 do
    begin
        score := 1;
        m := i mod rowlen;
        if (m = 0) or (m = (rowlen-1)) or (i < rowlen) or (i > ((rowc-1)*rowlen)) then
        begin
            continue
        end;

        k := i+1;
        sidescore := 0;
        while (k mod rowlen) <> 0 do { right side }
        begin
            inc(sidescore);
            if trees[k] >= trees[i] then
            begin
                if sidescore = 0 then
                    sidescore := 1;
                break
            end;
            inc(k);
        end;
        score := score * sidescore;

        k := i-1;
        sidescore := 0;
        while (k mod rowlen) <> (rowlen-1) do { left side }
        begin
            inc(sidescore);
            if trees[k] >= trees[i] then
            begin
                break
            end;
            dec(k);
        end;
        score := score * sidescore;

        k := i;
        sidescore := 0;
        dec(k, rowlen);
        while k > 0 do { up }
        begin
            inc(sidescore);
            if trees[k] >= trees[i] then
            begin
                break
            end;
            dec(k, rowlen);
        end;
        score := score * sidescore;

        k := i;
        inc(k, rowlen);
        sidescore := 0;
        while k < (rowlen*rowc) do { down }
        begin
            inc(sidescore);
            if trees[k] >= trees[i] then
            begin
                break
            end;
            inc(k, rowlen);
        end;
        score := score * sidescore;

        if score > maxscore then
            maxscore := score;
    end;

    writeln(maxscore)
end.
