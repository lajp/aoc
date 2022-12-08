program part1;

var trees: array[0..100000] of char;
var rowlen, rowc, i, ans, m, k: integer;
var c: char;
var visible: boolean;

begin
    i := 0;
    rowlen := 0;
    ans := 0;

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
        m := i mod rowlen;
        if (m = 0) or (m = (rowlen-1)) or (i < rowlen) or (i > ((rowc-1)*rowlen)) then
        begin
            inc(ans);
            continue
        end;

        visible := true;
        k := i+1;
        while (k mod rowlen) <> 0 do { right side }
        begin
            if trees[k] >= trees[i] then
            begin
                visible := false;
                break
            end;
            inc(k);
        end;
        if visible then
        begin
            inc(ans);
            continue
        end;

        k := i-1;
        visible := true;
        while (k mod rowlen) <> (rowlen-1) do { left side }
        begin
            if trees[k] >= trees[i] then
            begin
                visible := false;
                break
            end;
            dec(k);
        end;
        if visible then
        begin
            inc(ans);
            continue
        end;

        k := i;
        visible := true;
        while k > 0 do { up }
        begin
            dec(k, rowlen);
            if trees[k] >= trees[i] then
            begin
                visible := false;
                break
            end;
        end;
        if visible then
        begin
            inc(ans);
            continue
        end;

        k := i;
        visible := true;
        while k < (rowlen*rowc) do
        begin
            inc(k, rowlen);
            if trees[k] >= trees[i] then
            begin
                visible := false;
                break
            end;
        end;
        if visible then
        begin
            inc(ans);
            continue
        end;
    end;

    writeln(ans)

    {for i := 0 to rowlen*rowc do
    begin
        if ((i mod rowlen) = 0) and (i <> 0) then
            writeln;
        write(trees[i])
    end;}
end.
