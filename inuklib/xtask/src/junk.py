
import hashlib

class Hashes:
	buffer: Dict[Path, str]

	def __init__(self):
		self.buffer = {}

	def add(self, path: Path):
		self.buffer[path] = self._hash_file(path)

	def serialize(self):
		l = [(str(path), file_hash) for path, file_hash in self.buffer.items()]
		l = sorted(l, key=lambda x: x[0])
		return "\n".join([f"{file_hash}\t{path_str}" for path_str, file_hash in l])

	@staticmethod
	def _hash_file(path: Path):
		with open(path, "rb") as f:
			return hashlib.file_digest(f, "md5").hexdigest()


def hash_tree(root: Path):
	hashes = Hashes()
	for path in root.rglob('*'):
		if path.is_file():
			hashes.add(path)
	print(hashes.serialize())

hash_tree(Path("../../"))