# n: new rust project. 
n:
	./new.sh $(r)

# r: run rust project. 
r:
	./main.sh $(r)

# c: check rust project. 
c:
	./check.sh $(r)

.PHONY: n r c