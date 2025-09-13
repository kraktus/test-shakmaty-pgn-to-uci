import chess
import chess.pgn

EXAMPLE_PGN = "1. e4 e6 2. d4 d5 3. e5 c5 4. Nf3 Qb6 5. Bd3 Bd7 6. O-O Bb5 7. Re1 Bxd3 8. Qxd3 cxd4 9. a3 Nc6 10. b4 a6 11. Bb2 Rc8 12. Nbd2 Qc7 13. Nb3 Nge7 14. Nbxd4 Nxd4 15. Bxd4 Qxc2 16. Qe3 Nf5 17. Qf4 Nxd4 18. Qxd4 Be7 19. Qb6 O-O 20. Qxb7 Qc6 21. Qxc6 Rxc6 22. Rac1 Rfc8 23. Rxc6 Rxc6 24. Ra1 h6 25. Kf1 Kf8 26. Ke2 Rc2+ 27. Ke3 Ke8 28. Nd4 Rc3+ 29. Kd2 Rc4 30. Kd3 Kd7 31. Ra2 Kc7 32. Rc2 Rxc2 33. Nxc2 Kb6 34. Kd4 Kb5 35. Kc3 Bg5 36. g3 Bc1 37. Kb3 f6 38. exf6 gxf6 39. Nd4+ Kb6 40. Nxe6 Bd2 41. Ng7 Be1 42. f3 d4 43. Kc4 Bf2 44. Nf5 h5 45. Nxd4 Bxd4 46. Kxd4 Kb5 47. g4 h4 48. f4 Ka4 49. Kc4 Kxa3 50. g5"

def str_to_san(x: str):
    """Return a list of SAN moves (skip every 3rd token, i.e., move numbers)."""
    tokens = x.split()
    return [tokens[i] for i in range(len(tokens)) if i % 3 != 0]

def str_to_uci(pgn: str):
    """Convert a space-separated PGN string (with move numbers) to a space-separated UCI string."""
    san_list = str_to_san(pgn)
    board = chess.Board(chess960=True)
    uci_moves = []
    for san in san_list:
        move = board.parse_san(san)
        uci_moves.append(move.uci())
        board.push(move)
    return " ".join(uci_moves)


def test_benchmark_str_to_uci(benchmark):
    result = benchmark(str_to_uci, EXAMPLE_PGN)

    assert result == "e2e4 e7e6 d2d4 d7d5 e4e5 c7c5 g1f3 d8b6 f1d3 c8d7 e1h1 d7b5 f1e1 b5d3 d1d3 c5d4 a2a3 b8c6 b2b4 a7a6 c1b2 a8c8 b1d2 b6c7 d2b3 g8e7 b3d4 c6d4 b2d4 c7c2 d3e3 e7f5 e3f4 f5d4 f4d4 f8e7 d4b6 e8h8 b6b7 c2c6 b7c6 c8c6 a1c1 f8c8 c1c6 c8c6 e1a1 h7h6 g1f1 g8f8 f1e2 c6c2 e2e3 f8e8 f3d4 c2c3 e3d2 c3c4 d2d3 e8d7 a1a2 d7c7 a2c2 c4c2 d4c2 c7b6 d3d4 b6b5 d4c3 e7g5 g2g3 g5c1 c3b3 f7f6 e5f6 g7f6 c2d4 b5b6 d4e6 c1d2 e6g7 d2e1 f2f3 d5d4 b3c4 e1f2 g7f5 h6h5 f5d4 f2d4 c4d4 b6b5 g3g4 h5h4 f3f4 b5a4 d4c4 a4a3 g4g5"

def main():
    print(str_to_uci(EXAMPLE_PGN))


if __name__ == "__main__":
    main()
